


use reqwest::{Client,StatusCode};

#[tokio::main]
async fn main(){
    let  client = Client::builder()
                                    .build()
                                    .unwrap();

    let resp = client.get("https://localhost:8080/").send().await.unwrap();

    println!("Status code : {:?}",resp.status());

    let expexted = StatusCode::OK;
    // assert_eq!(resp.status(), expexted);
    // let json_val = resp.json::<serde_json::Value>().await.unwrap();
    // println!("json_val: {:?}", json_val);
    // assert!(json_val.get("result").is_some());


}