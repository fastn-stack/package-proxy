#[derive(serde::Deserialize, Debug)]
struct Query {
    url: String,
}

#[tracing::instrument]
#[actix_web::get("/")]
async fn index() -> impl actix_web::Responder {
    "ok"
}

#[tracing::instrument(skip(query))]
#[actix_web::get("/proxy/")]
async fn proxy(query: actix_web::web::Query<Query>) -> impl actix_web::Responder {
    tracing::info!(url = ?query.url);

    reqwest::get(&query.url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fastn_observer::observe();

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    println!("Listening on {}:{}", host, port);
    actix_web::HttpServer::new(|| actix_web::App::new().service(index).service(proxy))
        .bind((host, port))?
        .run()
        .await
}
