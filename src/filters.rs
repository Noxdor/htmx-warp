use warp::{filters::BoxedFilter, Filter};

pub fn use_hello_filter() -> BoxedFilter<(String,)> {
    warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {param}, whose agent is {agent}."))
        .boxed()
}
