use log::{error, info, warn};
use env_logger::Builder;

fn main() {
    Builder::new()
        .parse_env("MY_APP_LOG")
        .init();

    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");
}
