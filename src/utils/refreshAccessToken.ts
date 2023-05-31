import Util from "../Util.js";

export default async function refreshAccessToken() {
	console.log("Refreshing the access token...");
	const spotifyResponse = await Util.spotify.refreshAccessToken();
	Util.spotify.setAccessToken(spotifyResponse.body.access_token);
	console.log("Access token successfully refreshed.");

	setTimeout(refreshAccessToken, spotifyResponse.body.expires_in * 1000 - 60_000);
}
