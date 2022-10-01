#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateAnswerResponse {
    /// Created answer
    CreatedAnswer
    (models::Answer)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HealthResponse {
    /// Health check
    HealthCheck
    (models::Health)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListQuestionsResponse {
    /// List of all the users
    ListOfAllTheUsers
    (Vec<models::Question>)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Create a new answer
    async fn create_answer(
        &self,
        context: &C) -> Result<CreateAnswerResponse, ApiError>;

    /// Health check
    async fn health(
        &self,
        context: &C) -> Result<HealthResponse, ApiError>;

    /// List all questions
    async fn list_questions(
        &self,
        limit: Option<i64>,
        context: &C) -> Result<ListQuestionsResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Create a new answer
    async fn create_answer(
        &self,
        ) -> Result<CreateAnswerResponse, ApiError>;

    /// Health check
    async fn health(
        &self,
        ) -> Result<HealthResponse, ApiError>;

    /// List all questions
    async fn list_questions(
        &self,
        limit: Option<i64>,
        ) -> Result<ListQuestionsResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Create a new answer
    async fn create_answer(
        &self,
        ) -> Result<CreateAnswerResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_answer(&context).await
    }

    /// Health check
    async fn health(
        &self,
        ) -> Result<HealthResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().health(&context).await
    }

    /// List all questions
    async fn list_questions(
        &self,
        limit: Option<i64>,
        ) -> Result<ListQuestionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_questions(limit, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
