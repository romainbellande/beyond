mod fixtures;

use actix::{Actor, StreamHandler};
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenvy::dotenv;
use server::{get_app_state, setup_logs};

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    setup_logs();
    let app_state = get_app_state().await.clone();

    fixtures::execute_fixtures(app_state.db.clone()).await;

    log::info!("starting HTTP server at http://127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %U [%D ms][%s]"))
            .app_data(web::Data::new(app_state.clone()))
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
