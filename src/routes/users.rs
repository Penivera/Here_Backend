use actix_web::web;

/// Configure auth-related routes.
///
/// This registers endpoints from `crate::handlers::auth` (e.g. `/signup`).
pub fn init(cfg: &mut web::ServiceConfig) {
    // handlers are defined in the same crate root for the library; use crate to
    // resolve them when building the library crate
    use crate::handlers::users::signup;

    // Register user handlers under the "/users" prefix so endpoints become
    // e.g. POST /users/signup
    cfg.service(web::scope("/users").service(signup));
}
