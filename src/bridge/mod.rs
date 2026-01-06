// src/bridge/mod.rs 

pub mod cli;
pub mod rcl;

#[allow(dead_code)]
pub enum RosData {
    TopicList(Vec<String>),
    NodeList(Vec<String>),
}
