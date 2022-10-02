//! Main library entry point for relearnserver implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use relearnserver::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let service = relearnserver::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls())
                .expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM)
                .expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem")
                .expect("Failed to set certificate chain");
            ssl.check_private_key()
                .expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr)
            .serve(service)
            .await
            .unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server {
            marker: PhantomData,
        }
    }
}

use relearnserver::server::MakeService;
use relearnserver::{Api, CreateAnswerResponse, HealthResponse, ListQuestionsResponse};
use std::error::Error;
use swagger::ApiError;
#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Create a new answer
    async fn create_answer(&self, context: &C) -> Result<CreateAnswerResponse, ApiError> {
        let context = context.clone();
        info!("create_answer() - X-Span-ID: {:?}", context.get().0.clone());
        let res = CreateAnswerResponse::CreatedAnswer(models::Answer {
            created_at: None,
            org_id: Some(1),
            question_id: 1,
            valid: Some(true),
            user_id: Some(1),
        });
        Ok(res)
    }

    /// Health check
    async fn health(&self, context: &C) -> Result<HealthResponse, ApiError> {
        let context = context.clone();
        info!("health() - X-Span-ID: {:?}", context.get().0.clone());
        let res = HealthResponse::HealthCheck(models::Health {
            status: "ok".to_string(),
        });
        return Ok(res);
    }

    /// List all questions
    async fn list_questions(
        &self,
        _limit: Option<i64>,
        _context: &C,
    ) -> Result<ListQuestionsResponse, ApiError> {
        // get from environment variable

        let mut questions = Vec::new();
        for n in 1..101 {
            let question = models::Question {
                question: "What is the meaning of life?".to_string(),
                level: n.to_string(),
                number: Some(n),
                tag: "tag".to_string(),
                answers: Vec::new(),
                correct_answer: n,
                point_reward: n,
                tags: Some(vec!["tag".to_string()]),
                correct_response: Some("correct_response".to_string()),
                hint: "hint".to_string(),
                reference: Some(vec!["refs".to_string()]),
                wrong_response: Some("wrong_response".to_string()),
            };
            questions.push(question);
        }
        let list_question = ListQuestionsResponse::ListOfAllTheUsers(questions);
        return Ok(list_question);
    }
}
