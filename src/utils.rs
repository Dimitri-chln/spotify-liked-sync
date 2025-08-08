use std::{collections::HashMap, path::Path};

use serde::Deserialize;
use tokio::fs;

use crate::{Error, Result};

pub fn parse_query(query: &str) -> Result<HashMap<&str, &str>> {
    let mut query_map = HashMap::new();

    for item in query.split("&") {
        let (key, value) = item.split_once("=").ok_or(Error::InvalidQuery)?;
        query_map.insert(key, value);
    }

    Ok(query_map)
}

pub async fn load_json<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let content = fs::read_to_string(path).await?;
    let value = serde_json::from_str(&content)?;

    Ok(value)
}
