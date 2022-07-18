use mongodb::{options::ClientOptions, Client, Database, Collection, results::CollectionSpecificationInfo};
use std::env;

async fn connect() -> mongodb::error::Result<Database> {
    // set variables
    let mongo_url = env::var("mongo_url").unwrap_or("localhost".to_string());
    let mongo_port = env::var("mongo_port").unwrap_or("27017".to_string());
    let app_name = env::var("APP_NAME").expect("variable APP_NAME not specified");

    // set config
    let options = ClientOptions::parse(format!("mongodb://{mongo_url}:{mongo_port}")).await?;
    let client = Client::with_options(options)?;

    Ok(client.database(&app_name.to_string()))
}
