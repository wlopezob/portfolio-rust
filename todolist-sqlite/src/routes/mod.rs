use axum::{Json, Router, routing::get};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::{
        app_info,
        open_api::{self},
        settings::AppSettings,
    },
    controller::todo_controller::TodoController,
    types::AppState,
};

pub fn create_routes(app_settings: &AppSettings) -> OpenApiRouter<AppState> {
    let app_info = app_info::AppInfo::new();
    let openapi = open_api::configure_openapi(&app_info, app_settings);

    OpenApiRouter::with_openapi(openapi).nest("/todo", TodoController::router())
}

pub fn build_router(app_settings: &AppSettings, app_state: AppState) -> Router {
    let (api_router, api) = create_routes(app_settings).split_for_parts();
    let api_clone = api.clone();
    let openapi_json_path = format!("{}{}", app_settings.app.prefix, app_settings.openapi.json_path);
    let ui_path = app_settings.openapi.ui_path.clone();
    let openapi_json_path_for_route = app_settings.openapi.json_path.clone();

    let api_docs = Router::new()
        .merge(SwaggerUi::new(ui_path)
            .url(openapi_json_path, api))
        .route(
            openapi_json_path_for_route.as_str(),
            get(|| async move { Json(api_clone) }),
        )
        .merge(api_router);

    Router::new()
        .nest(&app_settings.app.prefix, api_docs)
        .with_state(app_state)
}
