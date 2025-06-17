


use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt
};
use std::time::Duration;
use tokio::io::AsyncReadExt;
use mongodb::bson::doc;
use url::Host;


pub fn get_db_connection_uri(host: Host, port: u16) -> String {
    format!("mongodb://{}:{}", host, port)
}

pub async fn get_mongo() -> ContainerAsync<GenericImage> {
    GenericImage::new("mongo", "4.4.1-bionic")
        .with_exposed_port(27017.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Waiting for connections"))
        .with_startup_timeout(Duration::from_secs(120))
        .start()
        .await
        .unwrap()
}

//   /// This function is used to provide a database connection to the tests
//   pub async fn get_db_connection(container: &ContainerAsync<GenericImage>) -> mongodb::Client {
//     let host = container.get_host().await.unwrap();
//     let port = container.get_host_port_ipv4(27017).await.unwrap();
//     mongodb::Client::with_uri_str(&get_db_connection_uri(host, port))
//         .await
//         .expect("Could not create database connection")
// }

   pub async fn get_db_connection(container: &ContainerAsync<GenericImage>) -> mongodb::Client {
        use tokio::time::{sleep, Duration};

        let host = container.get_host().await.unwrap();
        let port = container.get_host_port_ipv4(27017).await.unwrap();
        let uri = get_db_connection_uri(host, port);
        //let uri = format!("mongodb://{}:{}", host, port);

        let mut attempts = 0;
        let client;

        loop {
            match mongodb::Client::with_uri_str(&uri).await {
                Ok(c) => match c.database("admin").run_command(doc! {"ping": 1}).await {
                    Ok(_) => {
                        println!("✅ Mongo is ready!");
                        client = c;
                        break;
                    }
                    Err(e) => {
                        println!("⏳ Ping failed: {e}, retrying...");
                    }
                },
                Err(e) => {
                    println!("⏳ Connection failed: {e}, retrying...");
                }
            }

            attempts += 1;
            if attempts >= 10 {
                panic!("❌ MongoDB not ready after 10 attempts");
            }
            sleep(Duration::from_secs(1)).await;
        }

        client
    }

    #[allow(dead_code)]
    /// This function is used to print the stdout of a container for test debugging
    pub async fn print_stdout(container: &ContainerAsync<GenericImage>) {
        let mut reader_stdout = container.stdout(false);
        let mut dst = String::new();
        let _ = reader_stdout.read_to_string(&mut dst).await;
        println!("{}", dst);
    }

    #[allow(dead_code)]
    /// This function is used to print the stderr of a container for test debugging
    pub async fn print_stderr(container: &ContainerAsync<GenericImage>) {
        let mut reader_stderr = container.stderr(false);
        let mut dst = String::new();
        let _ = reader_stderr.read_to_string(&mut dst).await;
        println!("{}", dst);
    }