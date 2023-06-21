use std::collections::HashMap;

use actix_web::{
    App,    
    dev::{
        ServiceFactory, 
        ServiceRequest,
        ServiceResponse
    },
    Error, Route
};

use self::apub::ApubActivityHandler;

pub mod apub;
pub mod lemmy;
pub mod search;
pub mod utils;
