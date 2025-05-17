use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::controller;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::ping,

        controller::register,
        controller::login,
        controller::google,

        controller::insert_book,
        controller::get_book,
        controller::get_all_book,
        controller::update_book,
        controller::delete_book,
        controller::insert_book_review,
        controller::get_book_review,

        controller::insert_author,
        controller::get_author,
        controller::get_all_author,
        controller::delete_author,

        controller::insert_category,
        controller::get_category,
        controller::get_all_category,
        controller::update_category,
        controller::delete_category,

        controller::get_review,
        controller::update_review,
        controller::delete_review,
        controller::judge,
    ),
    modifiers(&SecurityAddon),
    components(schemas(crate::error::ErrorResponse))
)]
pub struct ApiDoc;
