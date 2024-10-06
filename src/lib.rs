mod object;
pub use object::Object;

mod cmd;
pub use cmd::Command;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
