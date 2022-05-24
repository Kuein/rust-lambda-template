use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{Value, json};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(lambda_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn lambda_handler(incoming: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _) = incoming.into_parts();
    println!("{:?}", event);
    let response = String::from("Hello world");
    Ok(json!({"message": response, "params": event}))
}
