use crate::handlers::users::*;
use crate::schemas::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        signup
    ),
    components(
        schemas(SignUp, SignShow)
    ),
    tags(
        (name = "H.E.R.E Backend Doc", description = "API documentation for H.E.R.E Backend")
    )
)]
pub struct ApiDoc;
