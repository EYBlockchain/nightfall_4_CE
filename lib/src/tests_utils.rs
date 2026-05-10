use log::{info, warn};
use mongodb::bson::doc;
use std::time::Duration;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use tokio::io::AsyncReadExt;
use url::Host;

const TEST_MONGO_REPLICA_SET_NAME: &str = "rs0";

pub fn get_db_connection_uri(host: Host, port: u16) -> String {
    format!(
        "mongodb://{host}:{port}/?replicaSet={TEST_MONGO_REPLICA_SET_NAME}&directConnection=true"
    )
}

fn get_direct_db_connection_uri(host: &Host, port: u16) -> String {
    format!("mongodb://{host}:{port}/?directConnection=true")
}

pub async fn get_mongo() -> ContainerAsync<GenericImage> {
    let mongo_image = GenericImage::new("mongo", "8.0")
        .with_exposed_port(27017.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Waiting for connections"))
        .with_entrypoint("mongod")
        .with_cmd(["--replSet", TEST_MONGO_REPLICA_SET_NAME, "--bind_ip_all"])
        .with_startup_timeout(Duration::from_secs(120));

    mongo_image.start().await.unwrap()
}

pub async fn get_db_connection(container: &ContainerAsync<GenericImage>) -> mongodb::Client {
    use tokio::time::{sleep, Duration};

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(27017).await.unwrap();
    let direct_uri = get_direct_db_connection_uri(&host, port);
    let rs_uri = get_db_connection_uri(host, port);

    let mut attempts = 0;
    loop {
        match mongodb::Client::with_uri_str(&direct_uri).await {
            Ok(c) => match c.database("admin").run_command(doc! {"ping": 1}).await {
                Ok(_) => {
                    info!(" Mongo is ready!");
                    break;
                }
                Err(e) => {
                    warn!("Ping failed: {e}, retrying...");
                }
            },
            Err(e) => {
                warn!("Connection failed: {e}, retrying...");
            }
        }

        attempts += 1;
        if attempts >= 10 {
            panic!(" MongoDB not ready after 10 attempts");
        }
        sleep(Duration::from_secs(1)).await;
    }

    let bootstrap_client = mongodb::Client::with_uri_str(&direct_uri).await.unwrap();
    initialize_replica_set(&bootstrap_client).await;
    wait_for_primary(&bootstrap_client).await;

    let client = mongodb::Client::with_uri_str(&rs_uri).await.unwrap();
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await
        .unwrap();
    client
}

async fn initialize_replica_set(client: &mongodb::Client) {
    match client
        .database("admin")
        .run_command(doc! {
            "replSetGetStatus": 1,
        })
        .await
    {
        Ok(_) => {
            info!("Mongo replica set already initialized");
        }
        Err(error) => {
            let should_initiate = replica_set_not_initialized(&error);

            if !should_initiate {
                panic!("Failed to read replica set status: {error}");
            }

            match client
                .database("admin")
                .run_command(doc! {
                    "replSetInitiate": {
                        "_id": TEST_MONGO_REPLICA_SET_NAME,
                        "members": [{
                            "_id": 0,
                            "host": "localhost:27017",
                        }],
                    },
                })
                .await
            {
                Ok(_) => info!("Initialized Mongo replica set"),
                Err(error) => {
                    let already_initialized = replica_set_already_initialized(&error);

                    if already_initialized {
                        info!("Mongo replica set was already initialized");
                    } else {
                        panic!("Failed to initialize Mongo replica set: {error}");
                    }
                }
            }
        }
    }
}

fn replica_set_not_initialized(error: &mongodb::error::Error) -> bool {
    let message = error.to_string();
    message.contains("NotYetInitialized")
        || message.contains("no replset config has been received")
        || message.contains("not yet initialized")
}

fn replica_set_already_initialized(error: &mongodb::error::Error) -> bool {
    let message = error.to_string();
    message.contains("already initialized") || message.contains("AlreadyInitialized")
}

async fn wait_for_primary(client: &mongodb::Client) {
    use tokio::time::{sleep, Duration};

    let mut attempts = 0;
    loop {
        match client
            .database("admin")
            .run_command(doc! {"hello": 1})
            .await
        {
            Ok(hello) if hello.get_bool("isWritablePrimary").unwrap_or(false) => {
                info!("Mongo replica set member is PRIMARY");
                return;
            }
            Ok(_) => warn!("Mongo replica set not PRIMARY yet, retrying..."),
            Err(error) => warn!("Failed to query hello during replica set init: {error}"),
        }

        attempts += 1;
        if attempts >= 30 {
            panic!("Mongo replica set did not reach PRIMARY state after 30 attempts");
        }

        sleep(Duration::from_secs(1)).await;
    }
}

#[allow(dead_code)]
/// This function is used to print the stdout of a container for test debugging
pub async fn print_stdout(container: &ContainerAsync<GenericImage>) {
    let mut reader_stdout = container.stdout(false);
    let mut dst = String::new();
    let _ = reader_stdout.read_to_string(&mut dst).await;
    println!("{dst}");
}

#[allow(dead_code)]
/// This function is used to print the stderr of a container for test debugging
pub async fn print_stderr(container: &ContainerAsync<GenericImage>) {
    let mut reader_stderr = container.stderr(false);
    let mut dst = String::new();
    let _ = reader_stderr.read_to_string(&mut dst).await;
    println!("{dst}");
}
