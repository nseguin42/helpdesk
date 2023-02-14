use helpdesk::server;
use helpdesk::utils::error::Error;
use helpdesk::utils::logger::setup_logger;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    setup_logger().await?;

    Ok(())
}
