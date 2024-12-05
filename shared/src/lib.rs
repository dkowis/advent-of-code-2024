mod errors;
mod input;

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::input::*;
    use std::sync::Once;

    pub use tracing::Level;
    pub use tracing::{debug, error, info, trace, warn};
    use tracing_subscriber::FmtSubscriber;

    static INIT: Once = Once::new();

    pub fn initialize_logger(user_level: Option<Level>) {
        INIT.call_once(|| {
            let mut level = Level::DEBUG;
            if let Some(set_level) = user_level {
                level = set_level;
            }
            let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

            tracing::subscriber::set_global_default(subscriber)
                .expect("Setting default subscriber failed!");
        });
    }
}
