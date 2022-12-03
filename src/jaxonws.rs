
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
            // todo
        }
        pub fn handle_potential_get_request(&self, stream: &TcpStream){
            // todo
        }
        pub fn handle_potential_post_request(&self, stream: &TcpStream){
            let request = self.read_request(&stream);
            let mut request = request.split_whitespace();
            let method = request.next().unwrap();
            let path = request.next().unwrap();
            let protocol = request.next().unwrap();
            if method == "POST" {
                if path == "/" {
                    self.write_html(&stream, "<h1>Index</h1>".to_owned());
                } else {
                    self.write_html(&stream, "<h1>404</h1>".to_owned());
                }
            }
        }
    }
}