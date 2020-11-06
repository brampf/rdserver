#[path = "model/user.rs"] mod user;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/version")]
async fn version() -> impl Responder {

    return "0.1";
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {

    let user = user::User::new(id, name);
    return user.to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .service(index)
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
