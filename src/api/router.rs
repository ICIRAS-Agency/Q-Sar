use std::net::SocketAddr;
use crate::qsar::{HttpMethod, HttpType};
use crate::utils::logger;

// Router main point
pub async fn route( request: Vec< String >, address: SocketAddr ) -> Result< u16, u16 >{
    // First header containing METHOD, PATH and HTTP version
    let first_header: Vec< &str > = request
        .first()
        .expect( "First index value should not be empty" )
        .split( " " )
        .collect();

    match HttpMethod::from( first_header.get( 0 ).unwrap().to_string().as_str() ) {
        HttpMethod::GET => {
            logger::write_access_log(format!(  "[GET]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("GET" ) ).await;
            println!( "[GET]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() );
            Ok( 200 )
        },
        HttpMethod::POST => {
            logger::write_access_log(format!( "[POST]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("POST" ) ).await;
            println!( "[POST]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() );
            Ok( 200 )
        },
        HttpMethod::PUT => {
            logger::write_access_log(format!( "[PUT]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("PUT" ) ).await;
            println!( "[PUT]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() );
            Ok( 200 )
        },
        HttpMethod::PATCH => {
            logger::write_access_log(format!( "[PATCH]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("PATCH" ) ).await;
            println!( "[PATCH]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() );
            Ok( 200 )
        },
        HttpMethod::DELETE => {
            logger::write_access_log(format!( "[DELETE]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() ), String::from( "HTTP" ), String::from("DELETE" ) ).await;
            println!( "[DELETE]\tIP - {}\tPATH: {}", address, first_header.get( 1 ).unwrap() );
            Ok( 200 )
        },
        _ => {
            println!( "Not a supported HTTP method" );
            Err( 400 )
        }
    }
}