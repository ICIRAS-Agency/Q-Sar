// Modules
pub mod api;
pub mod types;
pub mod utils;

pub mod qsar {
    // Dependencies
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt };
    use tokio::net::{ TcpListener, TcpStream };
    use tracing::{Level, Subscriber};
    use crate::api::router;
    use crate::types;
    use crate::utils::logger;

    // Structs
    pub struct Response {
        pub status: u16,
        pub headers: HashMap<String, String>,
        pub data: Box< dyn AsyncRead + Unpin + Send >
    }

    pub struct Request {
        pub method: types::http_method::HttpMethod,
        pub path: String,
        pub http_version: types::http_type::HttpType,
        pub headers: HashMap<String, String>,
        pub body: Vec<u8>
    }

    // Create server
    pub async fn create_server( host: &str, port: u16 ) -> Result< TcpListener, Box< dyn std::error::Error > > {
        // Init server
        logger::write_events_log( &format!( "Creating server and binding it to {}:{}.", &host, &port ), Level::INFO ).await;
        let full_host = format!( "{}:{}", host, port );
        let listener: Result< TcpListener, _ > = TcpListener::bind( &full_host ).await;

        match listener {
            Ok( tcpl ) => {
                logger::write_events_log( "Server started successfully!", Level::INFO ).await;
                Ok(tcpl)
            },
            Err( e ) => Err( Box::new( e ) )
        }

    }

    // Bind server and listen for incoming requests
    pub async fn listen( listener: TcpListener ) {
        logger::write_events_log( &format!( "Listening on {}", &listener.local_addr().unwrap() ), Level::INFO ).await;

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
        match stream.read( &mut buffer ).await {
            Ok( buf_size ) => {
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
                router::route( stream, request, addr ).await;       
            },
            Err( e ) => { 
                println!("Error reading from stream - {}", e ); 
            }
        }
    }
}