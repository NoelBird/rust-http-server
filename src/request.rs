use std::collections::HashMap;
#[allow(unused_imports)]

pub struct Request {
    method: String,
    http_version: String,
    query_param: HashMap<String, String>,
    header: HashMap<String, String>,
    body: String,
}

impl Request {

}