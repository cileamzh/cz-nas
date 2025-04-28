// use std::{
//     collections::HashMap,
//     sync::{Arc, Mutex},
// };

// use actix::{Actor, Context, Message};

// pub struct WsSession {
//     id: String,
//     addr: Arc<Mutex<HashMap<String, actix::Recipient<WsMsg>>>>,
// }

// impl Actor for WsSession {
//     type Context = Context<Self>;
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct WsMsg();
