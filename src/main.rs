#[derive(serde::Deserialize)]
struct Query {
    url: String,
}

#[actix_web::get("/")]
async fn index() -> impl actix_web::Responder {
    "ok"
}

#[actix_web::get("/proxy/")]
async fn proxy(query: actix_web::web::Query<Query>) -> impl actix_web::Responder {
    println!("proxying {}", query.url);
    let start = std::time::Instant::now();
    let res = reqwest::get(&query.url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    print_end("done", start);
    res
}

pub fn print_end(msg: &str, start: std::time::Instant) {
    use colored::Colorize;

    if fastn_core::utils::is_test() {
        println!("done in <omitted>");
    } else {
        println!(
            // TODO: instead of lots of spaces put proper erase current terminal line thing
            "\r{:?} {} in {:?}.                          ",
            std::time::Instant::now(),
            msg.green(),
            start.elapsed()
        );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
