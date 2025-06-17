mod utils;

use qsar_lib::qsar;
use qsar_lib::utils::logger;

#[tokio::main]
async fn main() {
    // Init logger
    let _guards = logger::init_logger();
    let server = qsar::create_server( "127.0.0.1", 3000 ).await;
    
    match server {
        Ok( addr ) => {
            qsar::listen( addr ).await;
        },
        Err( err ) => {}
    }
}