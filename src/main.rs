use std::io::Read;
use std::{error::Error, borrow::Borrow};
use std::fs::{self};
mod jaxonws;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const HOST: &str = "127.0.0.1";
    const PORT: i16 = 1444;

    let server = jaxonws::WebServer::new(HOST, PORT).await;
    for stream in server.incoming() {
    
        println!("{}", server.read_request(stream.as_ref().unwrap()));
        server.write_html(stream.as_ref().unwrap(), fs::read_to_string("html/index.html").unwrap());
        server.handle_potential_get_request(stream.as_ref().unwrap());
        server.handle_potential_post_request(stream.as_ref().unwrap());
    }

    return Ok(());
}