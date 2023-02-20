use grpc::dice_service_client::DiceServiceClient;
use grpc::{Dice, RollRequest};
use grpc::dice::{Face, OptionalDropN};

pub mod grpc {
    tonic::include_proto!("dice");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiceServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(RollRequest {
        dice: vec![
            Dice {
                faces: Face::Twenty.into(),
                count: 2,
                explodes_ons: Vec::new(),
                result: 0,
                optional_drop_n: Some(OptionalDropN::DropLowest(1)),
            },
            Dice {
                faces: Face::Six.into(),
                count: 2,
                explodes_ons: Vec::new(),
                result: 0,
                optional_drop_n: None,
            },
            Dice {
                faces: Face::Twelve.into(),
                count: 1,
                explodes_ons: Vec::new(),
                result: 0,
                optional_drop_n: None,
            },
        ],
    });

    let response = client.roll(request).await?;
    let results: Vec<i32> = response.get_ref().dice.iter().map(|die| die.result).collect();

    println!("RESPONSE={:?}", results);

    Ok(())
}
