use async_trait::async_trait;
use hyper::{Method, StatusCode};
use thiserror::Error;
use urlencoding::encode;

use crate::{
    akapi::{AkApiRoute, AkServer},
    error::AKApiError,
};

pub struct DeleteFlow;

#[async_trait]
impl AkApiRoute for DeleteFlow {
    type Body = String;
    type Response = ();
    type Error = DeleteFlowError;

    #[instrument]
    async fn send(
        api: &mut AkServer,
        api_key: &str,
        slug: Self::Body,
    ) -> Result<Self::Response, Self::Error> {
        let res = api
            .send(
                Method::DELETE,
                format!("/api/v3/flows/instances/{}/", encode(&slug)).as_str(),
                api_key,
                (),
            )
            .await?;

        match res.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Err(Self::Error::NotFound),
            code => Err(Self::Error::Unknown(format!(
                "Invalid status code {}",
                code
            ))),
        }
    }
}

#[derive(Error, Debug)]
pub enum DeleteFlowError {
    #[error("The given flow was not found.")]
    NotFound,
    #[error("An unknown error occured ({0}).")]
    Unknown(String),
    #[error(transparent)]
    RequestError(#[from] AKApiError),
}
