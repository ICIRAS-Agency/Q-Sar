// Dependencies
use dotenv::dotenv;
use std::env;
use std::fmt::Debug;
use std::string::ToString;
use async_graphql::futures_util::SinkExt;
use async_graphql::GuardExt;
use tracing;
use tracing::{event, span, Level, Metadata, Subscriber};
use tracing::instrument::WithSubscriber;
use tracing_subscriber;
use tracing_subscriber::prelude::*;
use tracing_subscriber::filter::{FilterExt, LevelFilter, LevelParseError};
use tracing_appender;
use tracing_appender::non_blocking::{ NonBlocking, WorkerGuard };
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::Layer;
use tracing_subscriber::util::SubscriberInitExt;
// Libs
use crate::qsar::{ HttpMethod, HttpType };

// Structs
pub struct LoggerGuards {
    access_guard: WorkerGuard,
    events_guard: WorkerGuard,
    debug_guard: WorkerGuard
}

// Base path to write logs to
pub fn init_logger() -> LoggerGuards {
    dotenv().ok();
    let path_str = env::var( "LOG_PATH" ).unwrap_or_else( | _e | "./logs/".to_string() );

    // Multiple tracing::level filter
    let multi_levels_filter = LevelFilter::INFO.or( LevelFilter::WARN ).or( LevelFilter::ERROR );

    // Initiating appender
    let access_file_appender = tracing_appender::rolling::daily( &path_str, "access-log" );
    let ( non_blocking_access, access_guard ) = tracing_appender::non_blocking( access_file_appender );
    let events_file_appender = tracing_appender::rolling::daily( &path_str, "events-log" );
    let ( non_blocking_events, events_guard ) = tracing_appender::non_blocking( events_file_appender );
    let debug_file_appender = tracing_appender::rolling::daily( &path_str, "debug-log" );
    let ( non_blocking_debug, debug_guard ) = tracing_appender::non_blocking( debug_file_appender );

    // Formating and writing layers
    let access_file_layer = tracing_subscriber::fmt::layer()
        .with_writer( non_blocking_access )
        .with_ansi( false )
        .with_level( true )
        .with_target( false )
        .with_filter( LevelFilter::TRACE );
    let events_file_layer = tracing_subscriber::fmt::layer()
        .with_writer( non_blocking_events )
        .with_ansi( false )
        .with_level( true )
        .with_target( false )
        .with_filter( multi_levels_filter );
    let debug_file_layer = tracing_subscriber::fmt::layer()
        .with_writer( non_blocking_debug )
        .with_ansi( false )
        .with_level( true )
        .with_target( false )
        .with_filter( LevelFilter::DEBUG );

    // Initialize global subscriber
    tracing_subscriber::registry()
        .with( access_file_layer )
        .with( events_file_layer )
        .with( debug_file_layer )
        .init();

    // Return guards
    LoggerGuards {
        access_guard,
        events_guard,
        debug_guard
    }
}

// Write HTTP requests to web_access_{date} file
pub async fn write_access_log( message: String, http_type: String, http_method: String ) {
    tracing::trace!( "[{}] - [{}]\t{}", http_type, http_method, message );
}

// TODO: Fix writing to file
// Write Error and Warnings to incidents_{date} file
pub async fn write_events_log( message: &str, log_type: Level ) {
    match log_type {
        Level::INFO => {
            tracing::info!( "{}", message );
        }
        Level::WARN => {
            tracing::warn!( "{}", message );
        }
        Level::ERROR => {
            tracing::error!( "{}", message );
        }
        _ => {
            eprintln!( "Not a valid event log, can only log: INFO, WARN, ERROR" );
        }
    }
}

// TODO: Add writing to debug file
// Write Debug information to bug_rapport_{data} file
pub async fn write_debug_log( message: String ) {
    tracing::debug!( "{}", message );
}
