use askama::Template;
use axum::response::IntoResponse;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate;

#[derive(Template)]
#[template(path = "overview.html")]
pub struct OverviewTemplate;

#[derive(Template)]
#[template(path = "console.html")]
pub struct ConsoleTemplate;

#[derive(Template)]
#[template(path = "players.html")]
pub struct PlayersTemplate;

pub async fn index() -> impl IntoResponse {
    BaseTemplate
}

pub async fn overview() -> impl IntoResponse {
    OverviewTemplate
}

pub async fn console() -> impl IntoResponse {
    ConsoleTemplate
}

pub async fn players() -> impl IntoResponse {
    PlayersTemplate
}

// Helper to turn Template errors into 500
#[allow(unused)]
struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
