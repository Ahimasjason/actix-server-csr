use openssl::ssl::{
    SslMethod, 
    SslAcceptor,
    SslFiletype, 
    SslAcceptorBuilder,
    SslVerifyMode
};

use actix_web::{get, App, HttpRequest, HttpServer, Responder, HttpResponse};
#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    
    HttpResponse::Ok()
        .json(serde_json::json!({"result":"sucess"}))
}


fn create_ssl_selector() -> SslAcceptorBuilder {

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())
                .unwrap();
    acceptor.set_verify(SslVerifyMode::FAIL_IF_NO_PEER_CERT);
    acceptor.set_private_key_file("certs/key.pem", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_file("certs/cert.pem", SslFiletype::PEM).unwrap();
    acceptor
}


#[actix_web::main]
async fn main() -> std::io::Result<()>  {

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:8080", create_ssl_selector())?
        .run()
        .await
}
