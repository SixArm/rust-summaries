use log::{info, warn, error, debug};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Example info message");
    warn!("Example warn message");
    error!("Example error message");
    debug!("Example debug message");
}
