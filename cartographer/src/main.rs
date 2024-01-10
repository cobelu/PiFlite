use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a warp filter for serving the HTML page
    let html_route = warp::path!("cartographer")
        .map(|| warp::reply::html(include_str!("resources/templates/index.html")));

    // Combine the routes
    let routes = html_route;

    // Run the warp server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}
