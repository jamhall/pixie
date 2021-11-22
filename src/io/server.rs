use std::error::Error;
use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

use crate::common::ApplicationError;
use crate::io::codec::CommandCodec;
use crate::io::Queue;
use crate::io::transport::{Command, CommandResponse};

pub struct Server {
    address: String,
    queue: Arc<Queue>,
}

impl Server {
    pub fn new(address: String) -> Self {
        let queue = Arc::new(Queue::default());
        Self {
            address,
            queue,
        }
    }

    pub async fn listen(&self) -> futures::io::Result<TcpListener> {
        TcpListener::bind(&self.address).await
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        match self.listen().await {
            Ok(listener) => {
                let queue = self.queue.clone();
                tokio::spawn(async move {
                    loop {
                        if let Ok((stream, _)) = listener.accept().await {
                            if let Ok(command) = Server::process_stream(stream).await {
                                queue.push(command);
                            }
                        }
                    }
                });
                Ok(())
            }
            Err(error) => {
                Err(ApplicationError::Transport(format!("Could not start server: {}", error)))
            }
        }
    }

    pub fn poll(&self) -> Option<Command> {
        self.queue.pop()
    }

    pub async fn process_stream(stream: TcpStream) -> Result<Command, Box<dyn Error>> {
        let (mut writer, mut reader) = Framed::new(stream, CommandCodec::default()).split();
        if let Some(request) = reader.next().await {
            return match request {
                Ok(command) => {
                    writer.send(CommandResponse::Accepted).await?;
                    Ok(command)
                }
                Err(error) => {
                    writer.send(CommandResponse::Refused("Could not process request".to_string())).await?;
                    Err(error.into())
                }
            };
        }

        Ok(Command::None)
    }
}

