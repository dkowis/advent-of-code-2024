mod errors;
mod input;

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::input::*;

    pub use flexi_logger::FlexiLoggerError;
    pub use flexi_logger::Logger;
    pub use log::{debug, error, info, trace, warn};
}
