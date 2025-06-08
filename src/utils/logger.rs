// Dependencies
use dotenv::dotenv;
use chrono;
use std::env;
use std::sync::OnceLock;
use std::string::ToString;
use tokio::sync::Mutex;
use tracing;
use tracing_subscriber;
use tracing_appender;

// Libs
use crate::qsar::{ HttpMethod, HttpType };

// Base path to write logs to
static LOG_PATH: OnceLock< Mutex< String > > = OnceLock::new();
pub fn init_logger() {
    LOG_PATH.get_or_init( || {
        let path_str = match env::var( "LOG_PATH" ) {
            Ok( path ) => path,
            Err( e ) => "./logs/".to_string()
        };

        Mutex::new( path_str )
    } );
}

// TODO: Fix writing to file
// Write HTTP requests to web_access_{date} file
pub async fn write_access_log( message: &str, http_type: &HttpType, http_method: &HttpMethod ) {
    let file_appender = tracing_appender::rolling::daily( LOG_PATH.get().unwrap().lock().await.as_str(), "access-log" );
    let ( non_blocking, _guard ) = tracing_appender::non_blocking( file_appender );
    let collector = tracing_subscriber::fmt().with_writer( non_blocking ).init();
    let timestamp = chrono::Local::now();

    match http_type {
        HttpType::HTTP => {
            match http_method {
                HttpMethod::GET => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTP - GET\t{1}", timestamp, message );
                },
                HttpMethod::POST => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTP - POST\t{1}", timestamp, message );
                },
                HttpMethod::PUT => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTP - PUT\t{1}", timestamp, message );
                },
                HttpMethod::DELETE => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTP - DELETE\t{1}", timestamp, message );
                },
                HttpMethod::PATCH => {},
                HttpMethod::OPTIONS => {},
                HttpMethod::CONNECT => {},
                HttpMethod::TRACE => {},
                _ => {}
            }
        }
        HttpType::HTTPS => {
            match http_method {
                HttpMethod::GET => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTPS - GET\t{1}", timestamp, message );
                },
                HttpMethod::POST => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTPS - POST\t{1}", timestamp, message );
                },
                HttpMethod::PUT => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTPS - PUT\t{1}", timestamp, message );
                },
                HttpMethod::DELETE => {
                    tracing::event!( tracing::Level::TRACE, "[{0}]\tHTTPS - DELETE\t{1}", timestamp, message );
                },

                HttpMethod::PATCH => {},
                HttpMethod::OPTIONS => {},
                HttpMethod::CONNECT => {},
                HttpMethod::TRACE => {},
                _ => {}
            }
        }
    }
}

// TODO: Fix writing to file
// Write Error and Warnings to incidents_{date} file
pub async fn write_events_log( message: &str, log_type: tracing::Level ) {
    let file_appender = tracing_appender::rolling::daily( LOG_PATH.get().unwrap().lock().await.as_str(), "incident-log" );
    let ( non_blocking, _guard ) = tracing_appender::non_blocking( file_appender );
    let collector = tracing_subscriber::fmt().with_writer( non_blocking ).init();
    let timestamp = chrono::Local::now();

    /*match log_type {
        tracing::Level::ERROR => {
            tracing::event!( tracing::Level::ERROR, "[{0}]\tERROR - {1}", timestamp, message );
        },
        tracing::Level::WARN => {
            tracing::event!( tracing::Level::WARN, "[{0}]\tWARNING - {1}", timestamp, message );
        },
        tracing::Level::INFO => {
            tracing::event!( tracing::Level::INFO, "[{0}]\tINFO - {1}", timestamp, message );
        }
    }*/
}
// TODO: Add writing to debug file
// Write Debug information to bug_rapport_{data} file
pub async fn write_debug_log() {
    let file_appender = tracing_appender::rolling::daily( LOG_PATH.get().unwrap().lock().await.as_str(), "debug-log" );
    let ( non_blocking, _guard ) = tracing_appender::non_blocking( file_appender );
    let collector = tracing_subscriber::fmt().with_writer( non_blocking ).init();
    let timestamp = chrono::Local::now();
}
