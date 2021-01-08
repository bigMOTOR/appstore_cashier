use warp::Filter;

pub fn make_routes() -> impl Filter<Extract = impl warp::Reply> + Clone {
    let root = warp::path("api");
    let v1 = warp::path("v1");

    // GET status
    let status = warp::path( "status")
        .map(|| "I am alive!");

    // GET verify
    let verify = warp::path( "verify")
        .map(|| "verify");

    let routes = warp::get()
        .and(root)
        .and(v1)
        .and(status
            .or(verify)
        );

    return routes;
}
