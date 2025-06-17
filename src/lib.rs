pub mod utils;

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

    // TODO: Create function that takes incoming connection and filters out the binary data
    async fn handle_connections( mut stream: TcpStream, addr: SocketAddr ) {
        // Buffer
        let mut buffer = vec![ 0u8; 2048 ];
        stream.read( &mut buffer ).await;

        // Read request and collect request headers
        let req_str = std::str::from_utf8( &buffer ); // Contains full request headers
        let mut lines: Vec< String > = Vec::new();
        match req_str {
            Ok( line ) => {
                lines = line.lines().map( | line | line.to_string() ).collect();
            },
            Err( e ) => {
                eprintln!( "Error reading request: {}", e );
            }
        }
        // First header containing METHOD, PATH and HTTP version
        let req_first: Vec< &str > = lines.first().expect( "First index value should not be empty" ).split( " " ).collect();
        
        // TODO: Route requests to the right resource + Write a match to check for HTTP / HTTPS
        match HttpMethod::from( req_first.get( 0 ).unwrap().to_string().as_str() ) {
            HttpMethod::GET => {
                logger::write_access_log(format!( "{}\t{}", &addr, req_first.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("GET" ) ).await;
                println!( "[HTTP] - [GET]\t{}\t{}", &addr, req_first.get( 1 ).unwrap() );
            },
            HttpMethod::POST => {
                logger::write_access_log(format!( "{}\t{}", &addr, req_first.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("POST" ) ).await;
                println!( "[HTTP] - [POST]\t{}\t{}", &addr, req_first.get( 1 ).unwrap() );
            },
            HttpMethod::PUT => {
                logger::write_access_log(format!( "{}\t{}", &addr, req_first.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("PUT" ) ).await;
                println!( "[HTTP] - [PUT]\t{}\t{}", &addr, req_first.get( 1 ).unwrap() );
            },
            HttpMethod::PATCH => {
                logger::write_access_log(format!( "{}\t{}", &addr, req_first.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("PATCH" ) ).await;
                println!( "[HTTP] - [PATCH]\t{}\t{}", &addr, req_first.get( 1 ).unwrap() );
            }
            HttpMethod::DELETE => {
                logger::write_access_log(format!( "{}\t{}", &addr, req_first.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("DELETE" ) ).await;
                println!( "[HTTP] - [DELETE]\t{}\t{}", &addr, req_first.get( 1 ).unwrap() );
            },
            _ => {
                println!( "Not supported HTTP method: {}", req_first[ 0 ] );
            }
        }
    }
}