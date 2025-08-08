use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use spotify_rs::{AuthCodeClient, RedirectUrl, Unauthenticated};
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::env::Env;
use crate::error::Transform as _;
use crate::{Error, Result, SCOPES, TOKEN_FILE, utils};

/// Auth code flow
pub async fn authorize() -> Result<()> {
    let env = Env::load()?;
    let redirect_uri = RedirectUrl::new(env.spotify_redirect_uri().to_owned()).transform()?;

    // Redirect the user to this URL
    let (client, url) = AuthCodeClient::new(
        env.spotify_client_id(),
        env.spotify_client_secret(),
        SCOPES,
        redirect_uri,
        true,
    );

    println!("Authorize the app at: {url}");

    // Start a small HTTP server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr).await?;
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);

    let service = AuthorizeCallback::new(client);
    let connection = http1::Builder::new()
        .keep_alive(false)
        .serve_connection(io, service);

    if let Err(error) = connection.await {
        eprintln!("Error serving connection: {error}");
    }

    Ok(())
}

struct AuthorizeCallback {
    client: Arc<Mutex<Option<AuthCodeClient<Unauthenticated>>>>,
}

impl AuthorizeCallback {
    pub fn new(client: AuthCodeClient<Unauthenticated>) -> Self {
        Self {
            client: Arc::new(Mutex::new(Some(client))),
        }
    }
}

impl Service<Request<Incoming>> for AuthorizeCallback {
    type Response = Response<Full<Bytes>>;
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let client = self.client.clone();

        Box::pin(async move {
            let query = req.uri().query().ok_or(Error::InvalidQuery)?;
            let query = utils::parse_query(query)?;
            let auth_code = *query.get("code").ok_or(Error::NoAuthCode)?;
            let csrf_state = *query.get("state").ok_or(Error::NoAuthCode)?;

            // Authenticate the client
            let client = client
                .lock()
                .await
                .take()
                .expect("The callback should only be called once");

            let spotify = client
                .authenticate(String::from(auth_code), String::from(csrf_state))
                .await?;

            let token = spotify.token();
            let token = token
                .read()
                .expect("The lock holding the token has been poisoned.")
                .clone();

            let json_token = serde_json::to_string(&token).unwrap();
            fs::write(TOKEN_FILE, json_token).await?;
            println!("The token has successfully been saved to {TOKEN_FILE}");

            Ok(Response::new(Full::new(Bytes::from(
                "<h1>You may close this page</h1><script>window.close()</script>",
            ))))
        })
    }
}
