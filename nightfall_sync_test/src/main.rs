use configuration::{logging::init_logging, settings::get_settings};
use log::{debug, info};
use nightfall_client::domain::entities::Proposer;
use reqwest::{StatusCode, Url};
use tokio::time;

#[tokio::main]
async fn main() {
    let settings = get_settings();
    init_logging(
        settings.nightfall_test.log_level.as_str(),
        settings.log_app_only,
    );
    info!("querying nightfall proposers for their current block number");
    let client = reqwest::Client::new();
    let url = Url::parse(&settings.nightfall_proposer.url)
        .unwrap()
        .join("/v1/blockdata")
        .unwrap();
    let res = client.get(url).send().await.unwrap();
    res.error_for_status_ref().unwrap();
    let response = res.json::<u64>().await.unwrap();
    debug!("Proposer 1 block number: {}", response);
    // Query the second proposer, and give it a chance to resync
    let mut response_2 = 0;
    let mut count = 0;
    while response_2 < response {
        let url = Url::parse("http://proposer2:3000")
            .unwrap()
            .join("/v1/blockdata")
            .unwrap();
        let res = client.get(url).send().await.unwrap();
        res.error_for_status_ref().unwrap();
        response_2 = res.json::<u64>().await.unwrap();
        time::sleep(time::Duration::from_secs(30)).await;
        count += 1;
        if count > 20 {
            panic!(
                "Proposer 2 is not syncing. Block count is {} versus {}",
                response_2, response
            );
        }
    }
    debug!("Proposer 2 block number: {}", response_2);
    // as a check, get a proposer list
    let url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("/v1/proposers")
        .unwrap();
    let res = client.get(url).send().await.unwrap();
    res.error_for_status_ref().unwrap();
    let list = res.json::<Vec<Proposer>>().await.unwrap();
    info!("Proposer list: {:?}", list);
    // if all is well, register the second proposer
    info!("Proposer 2 is synchronised. Registering proposer 2");
    let url = Url::parse("http://proposer2:3000")
        .unwrap()
        .join("/v1/register")
        .unwrap();
    let res = client
        .post(url)
        .json("http://proposer2:3000")
        .send()
        .await
        .unwrap();
    res.error_for_status_ref().unwrap();
    info!("Proposer 2 is now registered");
    // as a check, get a proposer list
    let url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("/v1/proposers")
        .unwrap();
    let res = client.get(url).send().await.unwrap();
    res.error_for_status_ref().unwrap();
    let list = res.json::<Vec<Proposer>>().await.unwrap();
    info!("Proposer list: {:?}", list);

    // and by now we should have done enough blocks to rotate to it
    info!("Rotating proposers");
    let url = Url::parse(&settings.nightfall_proposer.url)
        .unwrap()
        .join("/v1/rotate")
        .unwrap();
    let res = client.get(url).send().await.unwrap();
    res.error_for_status_ref().unwrap();
    info!("Proposers rotated");
    // as a check, get a proposer list
    let url = Url::parse(&settings.nightfall_client.url)
        .unwrap()
        .join("/v1/proposers")
        .unwrap();
    let res = client.get(url).send().await.unwrap();
    res.error_for_status_ref().unwrap();
    let list = res.json::<Vec<Proposer>>().await.unwrap();
    info!("Proposer list: {:?}", list);
    // rotating the proposer now, should fail
    info!("Rotating proposers again");
    let url = Url::parse(&settings.nightfall_proposer.url)
        .unwrap()
        .join("/v1/rotate")
        .unwrap();
    let res = client.get(url).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::LOCKED);
}
