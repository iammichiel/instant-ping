use actix::prelude::*;
use std::time::Duration;
use controllers::CertificateInformation;

pub struct CheckCertificateMessage{
    pub domain: String
}

impl Message for CheckCertificateMessage {
    type Result = Option<CertificateInformation>;
}

pub struct CertificateCheckActor;
impl Actor for CertificateCheckActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.heartbeat(ctx);
    }
}

impl CertificateCheckActor {
    fn heartbeat(&self, ctx: &mut Context<Self>) {
        println!("COUCOU  !");

        ctx.run_interval(Duration::new(5, 0), |act, ctx| {
            // act.framed.write(codec::ChatRequest::Ping);
            // act.hb(ctx);
            // act.send()
            // println!("This should run 5 seconds later");
        });
    }
}

impl Handler<CheckCertificateMessage> for CertificateCheckActor {
    type Result = Option<CertificateInformation>;

    fn handle(&mut self, msg: CheckCertificateMessage, ctx: &mut Context<Self>) -> Self::Result {

        let formatted_domain = format!("{}:443", msg.domain);
//     let connector = SslConnector::builder(SslMethod::tls()).unwrap().build(); 
//     let stream = TcpStream::connect(formatted_domain).unwrap();
//     let connection = connector.connect(&domain, stream).unwrap();

        println!("Received this message : {}", msg.domain);

        None
    }
}

// pub fn start_system() {
//     // Starting the actor system to run the check async
//     let system = System::new("certificate-checkers");
//     let addr = CertificateCheckActor{}.start();
//     let res = addr.send(CheckCertificateMessage{ domain: String::from("www.ouistock.fr")});

//     //  Arbiter::spawn(
//     //     res.map(|res| {
//     //         println!("RESULT: {}", res.is_some());
//     //     })
//     //     .map_err(|_| ()));

//     system.run();
// }
 
