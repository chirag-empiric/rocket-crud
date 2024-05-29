use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

#[tokio::main]
pub async fn connect() -> Result<Client, Box<dyn std::error::Error>> {
    // let db_uri = "mongodb+srv://chirag:JvAvSQUnWCt3e1vy@rust0.zcazvt1.mongodb.net/";
    let db_uri = "mongodb://localhost:27017/";
    let mut client_options = ClientOptions::parse_async(db_uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

    client_options.server_api = Some(server_api);

    let connection = Client::with_options(client_options)?;

    Ok(connection)
}
