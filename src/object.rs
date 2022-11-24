use crate::class::Class;
use std::{
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
};

/// Types that implement FromStr should use their FromStr implementation. Other types should use
/// ron (https://github.com/ron-rs/ron)
pub trait ObjectFromStr {
    fn from_str(s: &str) -> Result<Rc<dyn Object>, Box<dyn Error + Send + Sync>>
    where
        Self: Sized;
}

impl<T: 'static + FromStr + Object> ObjectFromStr for T
where
    T::Err: 'static + Error + Send + Sync,
{
    fn from_str(s: &str) -> Result<Rc<dyn Object>, Box<dyn Error + Send + Sync>> {
        <Self as FromStr>::from_str(s)
            .map_err(Into::into)
            .map(|o| Rc::new(o) as Rc<dyn Object>)
    }
}

/// The object of a data type. Data type is derived from the object's class. Methods specified here
/// are for use in nodes mostly.
pub trait Object: Display + Debug + ObjectFromStr {
    fn class(&self) -> Class;
    /// Since Object requires Display, this has little use and is implemented  through ToString,
    /// which is implemented for all types implementing Display. Left for consistency with
    /// as_number and other methods
    fn as_string(&self) -> String {
        self.to_string()
    }
    /// Convert to number
    fn as_number(&self) -> f64;
    /// Convert to boolean
    fn as_bool(&self) -> bool;
    /// Suggested implementation: Have a `HashMap<String, Rc<dyn Object>>` to manage fields.
    /// Default implementation is `unimplemented!()` because most types don't have fields.
    fn get_field(&self, _field: Rc<dyn Object>) -> Rc<dyn Object> {
        unimplemented!()
    }
    /// Suggested implementation: use `String::from` to convert `&str` to `String` and use that as
    /// insertion key. Default implementation is `unimplemented!()` because most types don't have
    /// fields.
    fn set_field(&mut self, _field: Rc<dyn Object>, _value: Rc<dyn Object>) {
        unimplemented!()
    }
}
