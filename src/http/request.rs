pub struct Request {
        path: String,
        query_string: Option<String>,
        method: super::method::Method, //use an enum for this, since there are only a limited number of methods
        }