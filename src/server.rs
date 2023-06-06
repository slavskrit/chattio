use std::time::Duration;

use chat::chat_service_server::{ChatService, ChatServiceServer};
use chat::{GetMessagesRequest, Messages};
use http::Method;
use prost_types::Timestamp;
use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::chat::messages;

mod chat {
    include!("chat.rs");

    // Add this
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("chat_descriptor");
}

#[derive(Default)]
pub struct ChatImpl {}

#[tonic::async_trait]
impl ChatService for ChatImpl {
    type GetMessagesStream = ReceiverStream<Result<Messages, Status>>;

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> Result<Response<Self::GetMessagesStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let name = request.into_inner().name;
        println!("request {name}");
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(10)).await;
                let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10000)
                    .map(char::from)
                    .collect();
                let reply = Messages {
                    message_type: messages::MessageType::Text.into(),
                    time: Option::Some(Timestamp {
                        seconds: 123,
                        nanos: 123,
                    }),
                    message: String::from("test"),
                };
                tx.send(Ok(reply)).await.unwrap_or_default();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = "50051";
    let addr = format!("[::1]:{port}").parse().unwrap();
    let chat = ChatImpl::default();
    let chat_service = ChatServiceServer::new(chat);

    println!("Starting server at {port}");

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(chat::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    // .expect("reflection service could not build");

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST])
        .allow_origin(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(reflection_service)
        .add_service(chat_service)
        .serve(addr)
        .await?;

    Ok(())
}