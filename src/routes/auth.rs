use actix_web::web;

/// Configure auth-related routes.
///
/// This registers endpoints from `crate::handlers::auth` (e.g. `/signup`).
pub fn init(cfg: &mut web::ServiceConfig) {
    // handlers are defined in the same crate root for the library; use crate to
    // resolve them when building the library crate
    use crate::handlers::auth::signup;

    // Register auth handlers under the "/auth" prefix so endpoints become
    // e.g. POST /auth/signup
    cfg.service(web::scope("/auth").service(signup));
}
