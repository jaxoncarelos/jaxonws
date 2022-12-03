use std::io::Read;
use std::{error::Error, borrow::Borrow};
use std::fs::{read_to_string, self};
mod jaxonws;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const HOST: &str = "127.0.0.1";
    const PORT: i16 = 8080;

    let server = jaxonws::WebServer::new(HOST, PORT).await;

    for stream in server.incoming() {
        if stream.is_ok() {
            println!("{}", server.borrow().read_request(stream.as_ref().unwrap()));
            server.handle_potential_get_request(stream.as_ref().unwrap());
            server.handle_potential_post_request(stream.as_ref().unwrap());
            server.write_html(stream.as_ref().unwrap(), fs::read_to_string("html/index.html").unwrap());
        }
    }

    return Ok(());
}