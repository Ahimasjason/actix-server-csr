

use reqwest::{Client, tls::Certificate, StatusCode};

#[tokio::main]
async fn main(){
    
    let cert_bytes = include_bytes!("../../certs/cert1/cert.pem");
    let cert = Certificate::from_pem(cert_bytes).unwrap();
    let  client = Client::builder()
                                    .https_only(true)
                                    .add_root_certificate(cert)
                                    .build()
                                    .unwrap();

    let resp = client.get("https://localhost:8082/").send().await.unwrap();

    println!("Status code : {:?}",resp.status());

    let expexted = StatusCode::OK;
    assert_eq!(resp.status(), expexted);
    let json_val = resp.json::<serde_json::Value>().await.unwrap();
    println!("json_val: {:?}", json_val);
    assert!(json_val.get("result").is_some());


}