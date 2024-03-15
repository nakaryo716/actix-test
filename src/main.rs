use std::{error::Error, net::{Ipv4Addr, SocketAddr}};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    
    HttpServer::new(|| {
        let share = Share::new("Hello");
        App::new()
            .app_data(web::Data::new(share))
            .service(hello)
            .service(get_contents)
            .service(user)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

struct Share<T> {
    contents: T,
}

impl<T> Share<T> {
    fn new(contents: T) -> Self {
        Self {
            contents,
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    let body = String::from("Hello Actix");
    HttpResponse::Ok().body(body)
}

#[get("/contents")]
async fn get_contents(contents: web::Data<Share<&str>>) -> impl Responder {
    let body = contents.contents.to_string();
    HttpResponse::Ok().body(body)
}

#[get("/user/{id}/{name}")]
async fn user(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();

    let body = format!("I got id: {}, name: {}", id, name);

    HttpResponse::Ok().body(body)
}