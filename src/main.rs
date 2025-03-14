use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn ola_amigo() -> impl Responder {
    "Olá, amigo, estamos on!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ola_amigo) // Registrando o endpoint
    })
    .bind("127.0.0.1:8080")? // Servidor rodando no localhost na porta 8080
    .run()
    .await
}
