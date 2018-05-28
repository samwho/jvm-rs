use class::Class;
use failure::Error;

pub trait ClassLoader {
    fn load(&mut self, name: &str) -> Result<&Class, Error>;
}
