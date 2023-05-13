use mongodb::{options, Client};

pub async fn new() -> mongodb::error::Result<mongodb::Database> {
    let uri = ""; // TODO: Get uri from env
    let mut cx_opt = options::ClientOptions::parse(uri).await?;

    let stable_api = options::ServerApi::builder()
        .version(options::ServerApiVersion::V1)
        .build();

    cx_opt.server_api = Some(stable_api);

    let cx = Client::with_options(cx_opt)?;
    let res = cx.database("test");
    Ok(res)
}
