use roller::roll;
use tonic::{transport::Server, Request, Response, Status};

use grpc::{RollRequest, RollResponse};
use grpc::dice_service_server::{DiceService, DiceServiceServer};

pub mod grpc {
    tonic::include_proto!("dice");
}

mod roller;
mod generator;

#[derive(Debug, Default)]
pub struct MyDiceService {}

#[tonic::async_trait]
impl DiceService for MyDiceService {
    async fn roll(&self, request: Request<RollRequest>) -> Result<Response<RollResponse>, Status> {
        let reply = grpc::RollResponse {
            dice: request.get_ref().dice.iter().map(|die| roll(die.clone(), generator::rand).unwrap_or_default()).collect(),
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
