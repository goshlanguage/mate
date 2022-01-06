use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use futures::executor::block_on;
use log::info;
use std::{time, thread};

async fn greet(req: HttpRequest) -> impl Responder {
    let now = time::Instant::now();
    let name = req.match_info().get("name").unwrap_or("World");
    info!("/greet executed: {:?}", now.elapsed().clone());
    format!("Hello {}!", name)
}


  async fn get_route(number: i8) -> i8 {
    return 1
  }

  #[actix_rt::main]
  async fn main() -> std::io::Result<()> {
  let now = time::Instant::now();
  let future_one = get_route(1);
  let outcome = block_on(future_one);
  thread::sleep(time::Duration::new(2, 0));
  println!("time elapsed {:?}", now.elapsed());
  println!("Here is the outcome: {}", outcome);
  HttpServer::new(move || {
      let app = App::new()
          .route("/", web::get().to(greet))
          .route("/{name}", web::get().to(greet));
      return app
  })
      .bind("127.0.0.1:8000")?
      .workers(3)
      .run()
      .await
}
