use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::LazyLock;
use regex::Regex;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::types::http_method::HttpMethod;
use crate::types::http_response::HttpResponse;

// Endpoint filter to retrieve all parts of the full path -- returns a vector of all path parts
async fn filter_endpoint( path: &str ) -> Vec< &str > {
    let path_without_query = path.split( '?' ).next().unwrap_or( path );

    let segments: Vec<&str> = path_without_query
        .split( '/' )
        .filter( | s | !s.is_empty() )
        .collect();

    segments
}

// Query filter to retrieve full query by user -- returns a vector of keys and values
async fn filter_query( query_string: &str ) -> Vec< Vec< String > > {
    // Split once on ? since a URI can only have one when querying
    let Some( (_path, query_part ) ) = query_string.split_once( '?' ) else {
        return Vec::new();
    };

    // If query is empty, return empty vec
    if query_part.is_empty() {
        return Vec::new();
    }

    // Filter query_part and split each query as a key/value in a multidimensional vector
    let filtered_queries: Vec< Vec< String > > = query_part
        .split( '&' )
        // Map over each pair in order to filter out key/value
        .map( | pair | {
            pair.split( '=' )
                // Make each &str a String
                .map( | s | s.to_string() )
                // Collect into a Vec< String >
                .collect()
    } ).collect::< Vec< Vec< String > > >();

    // Full filtered query
    filtered_queries
}

// Route action that calls the appropriate function based on the method that was provided in the request
async fn route_action( method: String, path: [ String; 2 ] ) -> Result< HttpResponse, HttpResponse > {
    match HttpMethod::from( method.as_str() ) {
        HttpMethod::GET => {
            // TODO: write logger
            Ok( HttpResponse { code: 200, headers: HashMap::new(), body: "".to_string() } )
        },
        HttpMethod::POST => {
            // TODO: write logger
            Ok( HttpResponse { code: 200, headers: HashMap::new(), body: "".to_string() } )
        },
        HttpMethod::PUT => {
            // TODO: write logger
            Ok( HttpResponse { code: 200, headers: HashMap::new(), body: "".to_string() } )
        },
        HttpMethod::PATCH => {
            // TODO: write logger
            Ok( HttpResponse { code: 200, headers: HashMap::new(), body: "".to_string() } )
        },
        HttpMethod::DELETE => {
            // TODO: write logger
            Ok( HttpResponse { code: 200, headers: HashMap::new(), body: "".to_string() } )
        },
        _ => {
            // TODO: write logger
            Err( HttpResponse { code: 404, headers: HashMap::new(), body: "".to_string() } )
        }
    }
}

// Router main point
// FIXME: Rewrite the route function as it does not support other type of endpoints than api endpoints
pub async fn route( mut stream: TcpStream, request: Vec< String >, address: SocketAddr ) {
    // First header containing METHOD, PATH and HTTP version
    let first_header: Vec< &str > = request
        .first()
        .expect( "First index value should not be empty" )
        .split( " " )
        .collect();
    // Filter out every single part of the path
    let filtered_path = filter_endpoint( first_header.get( 1 ).unwrap() ).await;
    let filtered_query = filter_query( first_header.get( 1 ).unwrap() ).await;
    
    // Temporary keep for debugging purposes
    println!("Path: {:?}", filtered_path);
    println!("Query: {:?}", filtered_query);

    // TODO: Write code that checks all parts of the path and matches it with a list of defined paths
    // TODO: If path exist it should call the route_action function and pass it the full path, on which it will call the appropriate method
    // FIXME: Handle None value (when user visits web app without entering a path)
    if filtered_path.get( 0 ).unwrap().to_owned() != "api" {
        match filtered_path.get( 0 ).unwrap().to_owned() {
            "index" => {
                let response = format!( "HTTP/1.1 200 ERROR\r\nContent-Type: text/html\r\n\r\n<h1>Welcome to the Index page</h1>\r\n\r\n" );
                stream.write_all( response.as_bytes() ).await.unwrap();
            },
            "posts" => {
                let response = format!( "HTTP/1.1 200 ERROR\r\nContent-Type: text/html\r\n\r\n<h1>Welcome to the Posts page</h1>\r\n\r\n" );
                stream.write_all( response.as_bytes() ).await.unwrap();
            },
            _ => {
                let response = format!( "HTTP/1.1 404 ERROR\r\nContent-Type: text/html\r\n\r\n<h1>Page does not exist</h1>\r\n\r\n" );
                stream.write_all( response.as_bytes() ).await.unwrap();
            }
        }
    } else {
        let response = format!( "HTTP/1.1 200 ERROR\r\nContent-Type: text/html\r\n\r\n<h1>Welcome to the API page</h1>\r\n\r\n" );
        stream.write_all( response.as_bytes() ).await.unwrap();
    }
}