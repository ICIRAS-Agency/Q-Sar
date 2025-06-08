mod utils;

pub mod qsar {
    // Dependencies
    use std::collections::HashMap;
    use std::error::Error;
    use std::net::SocketAddr;
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt };
    use tokio::net::{ TcpListener, TcpStream };
    use tracing::Level;
    use crate::utils::logger;

    // Enums
    #[derive(Debug)]
    pub enum HttpType {
        HTTP,
        HTTPS
    }
    impl From< &str > for HttpType {
        fn from( s: &str ) -> Self {
            match s {
                "http" => HttpType::HTTP,
                "https" => HttpType::HTTPS,
                _ => HttpType::HTTP
            }
        }
    }

    #[derive(Debug)]
    pub enum HttpMethod {
        GET,
        POST,
        PUT,
        DELETE,
        HEAD,
        OPTIONS,
        TRACE,
        CONNECT,
        PATCH,
        UNKNOWN
    }
    impl From< &str > for HttpMethod {
        fn from( s: &str ) -> Self {
            match s {
                "GET" => HttpMethod::GET,
                "POST" => HttpMethod::POST,
                "PUT" => HttpMethod::PUT,
                "DELETE" => HttpMethod::DELETE,
                "HEAD" => HttpMethod::HEAD,
                "OPTIONS" => HttpMethod::OPTIONS,
                "TRACE" => HttpMethod::TRACE,
                "CONNECT" => HttpMethod::CONNECT,
                "PATCH" => HttpMethod::PATCH,
                _ => HttpMethod::UNKNOWN
            }
        }
    }

    // Structs
    pub struct Response {
        pub status: u16,
        pub headers: HashMap<String, String>,
        pub data: Box< dyn AsyncRead + Unpin + Send >
    }

    pub struct Request {
        pub method: HttpMethod,
        pub path: String,
        pub http_version: HttpType,
        pub headers: HashMap<String, String>,
        pub body: Vec<u8>
    }

    // Create server
    pub async fn create_server( host: &str, port: u16 ) -> Result< TcpListener, Box< dyn std::error::Error > > {
        println!( "Creating server and binding it to {}:{}.", &host, &port );
        let full_host = format!( "{}:{}", host, port );
        let listener: Result< TcpListener, _ > = TcpListener::bind( &full_host ).await;
        logger::init_logger();

        match listener {
            Ok( tcpl ) => {
                println!("Server started successfully!");
                Ok(tcpl)
            },
            Err( e ) => Err( Box::new( e ) )
        }

    }

    // Bind server and listen for incoming requests
    pub async fn listen( listener: TcpListener ) {
        println!("Listening on {}", &listener.local_addr().unwrap());
        
        loop {
            match listener.accept().await {
                Ok( ( tcp_stream, addr ) ) => {
                   tokio::spawn( async move {
                       handle_connections( tcp_stream, addr ).await;
                   } );
                },
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    // TODO: Create function that takes incoming connection and filters out the binary data
    async fn handle_connections( mut stream: TcpStream, addr: SocketAddr ) {
        // Buffer
        let mut buffer = vec![0u8; 2048];
        stream.read(&mut buffer).await;

        // Read request and collect request headers
        let req_str = std::str::from_utf8(&buffer); // Contains full request headers
        let lines: Vec<String> = req_str.unwrap().lines().map(|line| line.to_string()).collect();
        let req_type: Vec<&str> = lines.first().unwrap().split(" ").collect();

        // TODO: Route requests to the right resource
        match HttpMethod::from(req_type[0]) {
            HttpMethod::GET => {
                println!("Got GET from {}", &addr);
            },
            HttpMethod::POST => {
                println!("Got POST from {}", &addr);
            },
            HttpMethod::PUT => {
                println!("Got PUT from {}", &addr);
            },
            HttpMethod::DELETE => {
                println!("Got DELETE from {}", &addr);
            },
            _ => { println!("Not supported HTTP method: {}", req_type[0]); }
        }
    }
}