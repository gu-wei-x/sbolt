mod welcome;
use rocket::Route;
use rocket::routes;

pub(crate) fn routes() -> Vec<Route> {
    routes![welcome::welcome]
}
