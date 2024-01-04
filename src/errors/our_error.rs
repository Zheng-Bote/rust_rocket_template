use rocket::http::Status;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct OurError {
    pub status: Status,
    pub message: String,
    debug: Option<Box<dyn Error>>,
}

impl fmt::Display for OurError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for OurError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if self.debug.is_some() {
            self.debug.as_ref().unwrap().source();
        }
        None
    }
}

impl OurError {
    fn new_error_with_status(
        status: Status,
        message: String,
        debug: Option<Box<dyn Error>>,
    ) -> Self {
        if debug.is_some() {
            log::error!("Error: {:?}", &debug);
        }
        OurError {
            status,
            message,
            debug,
        }
    }
    pub fn new_bad_request_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new_error_with_status(Status::BadRequest, message, debug)
    }

    pub fn new_not_found_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new_error_with_status(Status::NotFound, message, debug)
    }

    pub fn new_internal_server_error(message: String, debug: Option<Box<dyn Error>>) -> Self {
        Self::new_error_with_status(Status::InternalServerError, message, debug)
    }
}
