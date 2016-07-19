use std::error::Error as StdError;
use std::fmt;
use std::borrow::Cow;

use iron::{IronError, status};

#[derive(Debug)]
pub struct RouterError {
    status: status::Status,
    reason: Cow<'static, str>,
}

impl RouterError {
    pub fn new<S>(status: status::Status, reason: S) -> RouterError
        where S: Into<Cow<'static, str>>,
    {
        RouterError {
            status: status,
            reason: reason.into(),
        }
    }
}

impl From<RouterError> for IronError {
    fn from(e: RouterError) -> IronError {
        let status = e.status;
        IronError::new(e, status)
    }
}

impl StdError for RouterError {
    fn description(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.reason)
    }
}
