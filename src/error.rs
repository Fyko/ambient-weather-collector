use std::error::Error;

pub type BoxDynError = Box<dyn Error + Send + Sync>;
