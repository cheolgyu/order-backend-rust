use actix_web::client::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};

pub struct SSLClinet {
    client: Client,
}

impl SSLClinet {
    pub fn build() -> Client {
        // disable ssl verification
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_verify(SslVerifyMode::NONE);
        let _ = builder
            .set_alpn_protos(b"\x02h2\x08http/1.1")
            .map_err(|e| eprintln!("ssl clinet build err: ==>:{:?}", e));

        Client::build()
            .connector(Connector::new().ssl(builder.build()).finish())
            .finish()
    }
}
