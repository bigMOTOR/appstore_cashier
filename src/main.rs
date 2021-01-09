use warp;

mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let routes = routes::make_routes();
    let addr = ([127, 0, 0, 1], 3030);
    println!("Listening on http://{:?}", addr);

    warp::serve(routes)
        .run(addr)
        .await;
}
