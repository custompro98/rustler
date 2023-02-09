use tonic::{transport::Server, Request, Response, Status};

use dice::{RollRequest, RollResponse};
use dice::dice_service_server::{DiceService, DiceServiceServer};

pub mod dice {
    tonic::include_proto!("dice");
}

mod roller;

#[derive(Debug, Default)]
pub struct MyDiceService {}

#[tonic::async_trait]
impl DiceService for MyDiceService {
    async fn roll(&self, request: Request<RollRequest>) -> Result<Response<RollResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = dice::RollResponse {
            dice: request.into_inner().dice.into(),
        };

        return Ok(Response::new(reply));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyDiceService::default();

    Server::builder()
        .add_service(DiceServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
