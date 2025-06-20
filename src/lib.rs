pub mod utils;
pub mod api;

pub mod qsar {
    // Dependencies
    use std::collections::HashMap;
    use std::error::Error;
    use std::net::SocketAddr;
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt };
    use tokio::net::{ TcpListener, TcpStream };
    use tracing::{Level, Subscriber};
    use tracing_appender::non_blocking::NonBlocking;
    use tracing_subscriber::util::SubscriberInitExt;
    use crate::utils;
    use crate::utils::logger;
    use crate::utils::logger::{init_logger, write_events_log};
    use crate::api::router;

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
        // Init server
        write_events_log( &format!( "Creating server and binding it to {}:{}.", &host, &port ), Level::INFO ).await;
        let full_host = format!( "{}:{}", host, port );
        let listener: Result< TcpListener, _ > = TcpListener::bind( &full_host ).await;

        match listener {
            Ok( tcpl ) => {
                write_events_log( "Server started successfully!", Level::INFO ).await;
                Ok(tcpl)
            },
            Err( e ) => Err( Box::new( e ) )
        }

    }

    // Bind server and listen for incoming requests
    pub async fn listen( listener: TcpListener ) {
        write_events_log( &format!( "Listening on {}", &listener.local_addr().unwrap() ), Level::INFO ).await;

        loop {
            match listener.accept().await {
                Ok( ( tcp_stream, addr ) ) => {
                   tokio::spawn( async move {
                       handle_connections( tcp_stream, addr ).await;
                   } );
                },
                Err(e) => {
                    eprintln!( "Error accepting connection: {}", e );
                }
            }
        }
    }

    // Handle connections
    async fn handle_connections( mut stream: TcpStream, addr: SocketAddr ) {
        // Buffer
        let mut buffer = vec![ 0u8; 2048 ];
        stream.read( &mut buffer ).await;

        // Read request and collect request headers
        let req_str = std::str::from_utf8( &buffer ); // Contains full request headers
        let mut request: Vec< String > = Vec::new();
        match req_str {
            Ok( line ) => {
                request  = line.lines().map( | line | line.to_string() ).collect();
            },
            Err( e ) => {
                eprintln!( "Error reading request: {}", e );
            }
        }
        
        // If request ok send back 200 series HTTP code, else 400 series HTTP code
        match router::route( request, addr ).await {
            Ok( code ) => {
                let response = format!( "HTTP/1.1 {} OK\r\n\r\n", code );
                stream.write_all( response.as_bytes() ).await;
            },
            Err( e ) => {
                let response = format!( "HTTP/1.1 {} Bad Request\r\n\r\n", e );
                stream.write_all( response.as_bytes() ).await;
            }
        }
    }
}