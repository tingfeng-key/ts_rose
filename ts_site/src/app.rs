mod http {
    fn index(req: &HttpRequest) -> impl Responder {
        "Hello world!"
    }

    pub fn start() {
        let app = App::new()
            .prefix("/app")
            .resource("/index.html", |r| r.method(Method::GET).f(index))
            .finish();
    }
}

mod ws {
    use actix::*;
    use actix_web::*;

    /// Define http actor
    struct Ws;

    impl Actor for Ws {
        type Context = ws::WebsocketContext<Self>;
    }

    /// Handler for ws::Message message
    impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {

        fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
            match msg {
                ws::Message::Ping(msg) => ctx.pong(&msg),
                ws::Message::Text(text) => ctx.text(text),
                ws::Message::Binary(bin) => ctx.binary(bin),
                _ => (),
            }
        }
    }

    pub fn start() {
        App::new()
            .resource("/ws/", |r| r.f(|req| ws::start(req, Ws)))
            .finish();
    }
}