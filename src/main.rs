mod filters;

use warp::Filter;

#[tokio::main]
async fn main() {

    let page = 
        warp::filters::fs::dir("./page");

    let partials = warp::path("p");

    let clicked = partials
        .and(warp::path("clicked"))
        .and(warp::filters::fs::file("./partials/message.html"));

    let not_found = warp::filters::fs::file("./partials/404.html");

    let app = page.or(clicked).or(not_found);

    println!("Starting server");
    warp::serve(app).run(([0, 0, 0, 0], 3030)).await;
}
