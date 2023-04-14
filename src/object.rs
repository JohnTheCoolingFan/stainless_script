use crate::class::Class;
use std::{
    cmp::Ordering,
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
};

/// Types that implement FromStr should use their FromStr implementation. Other types should use
/// ron (<https://github.com/ron-rs/ron>)
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

/// Stainless Script Object version of [`PartialEq`]
pub trait ObjectPartialEq {
    fn eq(&self, other: Rc<dyn Object>) -> bool;
    fn ne(&self, other: Rc<dyn Object>) -> bool {
        !self.eq(other)
    }
}

/// Stainless Script Object version of [`PartialOrd`]
pub trait ObjectPartialOrd {
    fn partial_cmp(&self, other: Rc<dyn Object>) -> Option<Ordering>;
    fn lt(&self, other: Rc<dyn Object>) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }
    fn le(&self, other: Rc<dyn Object>) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }
    fn gt(&self, other: Rc<dyn Object>) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }
    fn ge(&self, other: Rc<dyn Object>) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

/// Stainless Script Object version of [`Eq`]
pub trait ObjectEq: ObjectPartialEq {}

/// Stainless Script Object version of [`Ord`]
pub trait ObjectOrd: ObjectEq + ObjectPartialOrd {
    fn cmp(&self, other: Rc<dyn Object>) -> Ordering;
}

/// The object of a data type. Data type is derived from the object's class. Methods specified here
/// are for use in nodes mostly.
pub trait Object:
    Display + Debug + ObjectFromStr + ObjectPartialEq + ObjectPartialOrd + ObjectEq + ObjectOrd
{
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

    fn cast_to(&self, to: &Class) -> Rc<dyn Object> {
        if self.class().name == "any" {
            (to.obj_from_str.unwrap())(&self.as_string()).unwrap()
        } else {
            unimplemented!()
        }
    }
}
