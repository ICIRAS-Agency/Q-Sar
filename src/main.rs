use qsar_lib::qsar;

#[tokio::main]
async fn main() {
    let server = qsar::create_server( "127.0.0.1", 3000 ).await;
    match server {
        Ok( addr ) => {
            qsar::listen( addr ).await;
        },
        Err( err ) => {}
    }
}