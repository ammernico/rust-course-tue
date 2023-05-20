use askama::Template;
#[allow(unused_imports)]
use axum::{
    extract::Form,
    http::StatusCode,
    response::IntoResponse,
    response::{Html, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Template, Default)]
#[template(path = "form.html")]
struct UserInput<'a> {
    height: &'a str,
    weight: &'a str,
}

#[derive(Deserialize)]
struct UserInputFields {
    height: f32,
    weight: f32,
}

#[derive(Template)]
#[template(path = "success.html")]
struct SuccessTemplate<'a> {
    bmi: &'a str,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let router = Router::new()
        .route("/", get(parse_user_form))
        .route("/submit", post(submit));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn render_template(template: impl Template) -> Response {
    match template.render() {
        Ok(rendered) => Html(rendered).into_response(),
        Err(e) => {
            eprintln!("Failed to render template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn parse_user_form() -> Response {
    let bmi_template = UserInput::default();
    render_template(bmi_template)
}

async fn submit(Form(fields): Form<UserInputFields>) -> Response {
    println!("Height: {} Weight: {}\n", &fields.height, &fields.weight);

    let bmi = fields.weight / (fields.height * fields.height);
    let result = format!("{}", bmi);

    let template = SuccessTemplate { bmi: &result };
    render_template(template)
}
