use crate::clients::Clients;
use crate::config::Config;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Renewm {
    clients: Arc<Mutex<Clients>>,
    config: Arc<Config>,
}
