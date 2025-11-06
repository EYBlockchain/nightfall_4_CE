/// A module containing uncategorised functions used by more than one component
use configuration::settings::Settings;
use url::Url;
use warp::hyper::body::Bytes;

/// function to pull the proving key or deposit proving key from the server as a byte array
pub fn load_key_from_server(key_file: &str) -> Option<Bytes> {
    let settings = Settings::new().expect("Could not get settings");
    let url = Url::parse(&settings.configuration_url)
        .expect("Could not parse URL")
        .join(key_file)
        .unwrap();
    let client = reqwest::blocking::Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()
    .ok()?;

    let response = client.get(url).send().ok()?;
    let bytes = response.bytes().ok()?;
    Some(bytes)
}

/// function to drop a database
pub async fn drop_database(db_url: &str, db_name: &str) -> Result<(), mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str(db_url).await?;
    client.database(db_name).drop().await
}

/// function to drop a collection
pub async fn drop_collection<C: Send + Sync>(
    db_url: &str,
    db_name: &str,
    collection_name: &str,
) -> Result<(), mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str(db_url).await?;
    client
        .database(db_name)
        .collection::<C>(collection_name)
        .drop()
        .await
}
