use warp::Filter;
use super::handlers::receipt_handler;

pub fn make_routes() -> impl Filter<Extract = impl warp::Reply> + Clone {
    let root = warp::path("api");
    let v1 = warp::path("v1");

    // GET status
    let status = warp::path( "status")
        .map(|| "I am alive!");

    // POST verify
    let verify = warp::path( "verify")
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::path::end())
        .and(warp::body::json::<receipt_handler::ReceiptPayload>())
        // .map(receipt_handler::verify);
        .map(|payload| {
            format!("{:?}", payload)
        });

    let routes = root.and(v1)
        .and(
            warp::get().and(status)
                .or(warp::post().and(verify))
        );

    return routes;
}
