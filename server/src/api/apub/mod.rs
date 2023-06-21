pub mod activity;

use crate::database::DatabasePool;
use std::{
    collections::HashMap, 
    sync::Mutex
};
use actix_web::{
    web::{
        get, 
        Data
    }, 
    Responder, 
    Result, 
    Route
};

pub struct ApubActivityHandler {
    pub routes : HashMap<String, Route>
}

impl ApubActivityHandler {
    pub fn new() -> Self {
        let mut routes = HashMap::<String, Route>::new();
        routes.insert("/inbox".to_string(), get().to(Self::inbox));

        Self {
            routes
        }
    }

    pub async fn inbox<'a>(
        _pool : Data<Mutex<DatabasePool>>,
    ) -> Result<impl Responder> {
        Ok("")
    }
}
