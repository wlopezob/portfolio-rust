
use utoipa::{OpenApi};

use crate::config::{app_info::AppInfo, settings::AppSettings};

pub const TAG_TODO: &str = "Todo";
pub const TAG_TODO_DESC: &str = "Todo management endpoints";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = TAG_TODO, description = TAG_TODO_DESC)
    )
)]
pub struct ApiDoc;

pub fn configure_openapi(app_info: &AppInfo, app_settings: &AppSettings) ->  utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();

    let info = app_info.clone();
    doc.info.title = info.name;
    doc.info.version = info.version;
    doc.info.description = Some(info.description);
    doc.servers = Some(vec![utoipa::openapi::ServerBuilder::new()
        .url(format!("http://localhost:{}{}", app_settings.server.port, app_settings.app.prefix))
        .build()]);
    doc
}