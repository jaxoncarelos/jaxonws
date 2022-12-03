
pub mod WebServer {
    use std::net::{TcpListener, TcpStream, Incoming};
    use std::io::{Read, Write};
    pub async fn new(url: &str, port: i16) -> privWebServer {
        let endpoint = url.to_owned() + ":" + &port.to_string();
        let listener = TcpListener::bind(endpoint).unwrap();

        return privWebServer::new(listener);
    }

    pub struct privWebServer {
        server: TcpListener,
    }
    impl privWebServer {
        
        pub fn new(server: TcpListener) -> Self{
            Self {server}
        }
        pub fn read_request(&self, mut stream: &TcpStream) -> String{
            let mut buffer = [0; 1024];

            stream.read(&mut buffer).unwrap();
            
            let r#final = String::from_utf8_lossy(&buffer[..]).as_ref().to_owned();
            // return final
            r#final
        }
        pub fn incoming(&self) -> Incoming {
            return self.server.incoming();
        }
        pub fn write_request(&self, mut stream: &TcpStream, response: String){
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        pub fn write_html(&self, mut stream: &TcpStream, html: String){
            let response = "HTTP/1.1 200 OK\r\n\r\n".to_owned() + &html;
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        pub fn handle_potential_get_request(&self, stream: &TcpStream){
            let request = self.read_request(&stream);
            if request.contains("GET") {
                self.write_html(stream, "Hello World!".to_owned());
            }
        }
        pub fn handle_potential_post_request(&self, stream: &TcpStream){
            let request = self.read_request(&stream);
            if request.contains("POST") {
                self.write_html(stream, "Hello World!".to_owned());
            }
        }
    }
}