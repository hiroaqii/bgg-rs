use std::collections::HashMap;

use reqwest::{
    header::{self},
    Client, Response, StatusCode,
};

use crate::{
    errors::{BggError, HttpResponseError},
    response::{BggResult, Item},
};

pub struct BggClient {
    client: Client,
}

impl Default for BggClient {
    fn default() -> Self {
        BggClient {
            client: Client::new(),
        }
    }
}

impl BggClient {
    pub fn new() -> Self {
        BggClient::default()
    }

    pub fn with_client(client: Client) -> Self {
        BggClient { client }
    }

    pub async fn adv_search(self, query: &Vec<(String, String)>) -> BggResult<HashMap<String, Vec<Item>>> {
        let res = self
            .client
            .get("https://www.boardgamegeek.com/geeksearch.php")
            .header(header::ACCEPT, "application/json")
            .query(query)
            .send()
            .await
            .map_err(|err| BggError::HttpResponse(HttpResponseError::ReqwestError(err)))?;

        self.handler(res).await
    }

    async fn handler(&self, res: Response) -> BggResult<HashMap<String, Vec<Item>>> {
        dbg!("response = {}", &res);
        match res.status() {
            StatusCode::OK => res
                .json::<HashMap<String, Vec<Item>>>()
                .await
                .map_err(|err| {
                    BggError::HttpResponse(HttpResponseError::Various(format!("{}", err)))
                }),
            StatusCode::FORBIDDEN => Err(BggError::HttpResponse(HttpResponseError::Forbidden)),
            StatusCode::INTERNAL_SERVER_ERROR => Err(BggError::HttpResponse(
                HttpResponseError::InternalServerError,
            )),
            StatusCode::SERVICE_UNAVAILABLE => Err(BggError::HttpResponse(
                HttpResponseError::ServiceUnavailable,
            )),
            s => Err(BggError::HttpResponse(HttpResponseError::Various(format!(
                "Unexpected response received: {:?} (status code)",
                s
            )))),
        }
    }
}
