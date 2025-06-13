// Dependencies
use dotenv::dotenv;
use std::env;
use std::fmt::Debug;
use std::ptr::fn_addr_eq;
use std::sync::OnceLock;
use std::string::ToString;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tracing;
use tracing::{event, span, Level, Subscriber};
use tracing::log::SetLoggerError;
use tracing_subscriber;
use tracing_appender;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::util::SubscriberInitExt;
// Libs
use crate::qsar::{ HttpMethod, HttpType };

// Base path to write logs to
// FIXME: Does not seem to look for environment variable in .env... needs to be fixed
pub fn init_logger() {
    let path_str = match env::var( "LOG_PATH" ) {
        Ok( path ) => path,
        Err( _e ) => "./logs/".to_string()
    };
    tracing_appender::rolling::daily( &path_str, "access-log" );
    tracing_appender::rolling::daily( &path_str, "incidents-log" );
    tracing_appender::rolling::daily( &path_str, "debug-log" );
}

// Write HTTP requests to web_access_{date} file
pub async fn write_access_log( message: String, http_type: &HttpType, http_method: &HttpMethod ) {
    let file_access_appender = tracing_appender::rolling::daily( "./logs/", "access-log" );
    let ( access_non_blocking, _access_guard ) = tracing_appender::non_blocking( file_access_appender );
    let subscriber = tracing_subscriber::fmt().with_writer( access_non_blocking ).with_target( false ).with_level( true  ).with_ansi( false ).finish();
    subscriber.init();
    tracing::info!("{}", message);
}

// TODO: Fix writing to file
// Write Error and Warnings to incidents_{date} file
pub async fn write_events_log( non_blocking: NonBlocking, message: &str, log_type: tracing::Level ) {
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
pub async fn write_debug_log( non_blocking: NonBlocking, ) {
    let ( non_blocking, _guard ) = tracing_appender::non_blocking( non_blocking );
    let collector = tracing_subscriber::fmt().with_writer( non_blocking ).init();
    let timestamp = chrono::Local::now();
}
