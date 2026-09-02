use hhm_contracts::{valid_kind, PRODUCT};
use hhm_leptos_lambdas::{html_response, validate_get};
use lambda_http::{http::StatusCode, run, service_fn, tracing, Body, Error, Request, Response};
use leptos::prelude::*;
use leptos::tachys::view::RenderHtml;

const EVENT_KIND: &str = "occupancy.report";

async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
    let bounds = match validate_get(&request, 24, 250) {
        Ok(bounds) => bounds,
        Err(response) => return Ok(response),
    };

    if !valid_kind(EVENT_KIND) {
        return html_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "<!doctype html><title>contract error</title>".to_owned(),
        );
    }

    let page = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"Hacker House occupancy report"</title>
            </head>
            <body>
                <main>
                    <h1>"Hacker House occupancy report"</h1>
                    <p>"Product contract: " <code>{PRODUCT}</code></p>
                    <p>"Event kind: " <code>{EVENT_KIND}</code></p>
                    <p>"Bounded report rows requested: " <strong>{bounds.limit}</strong></p>
                    <p>
                        "This isolated Lambda entrypoint is ready for the shared-core "
                        "query adapter; it does not initialize an ORM connection during cold start."
                    </p>
                </main>
            </body>
        </html>
    }
    .to_html();

    html_response(StatusCode::OK, format!("<!doctype html>{page}"))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::Method;

    #[tokio::test]
    async fn renders_the_shared_product_contract() {
        let request = Request::builder()
            .method(Method::GET)
            .body(Body::Empty)
            .expect("valid request");

        let response = function_handler(request).await.expect("handler succeeds");
        assert_eq!(response.status(), StatusCode::OK);

        let Body::Text(body) = response.body() else {
            panic!("expected text response");
        };
        assert!(body.contains(PRODUCT));
        assert!(body.contains(EVENT_KIND));
    }
}
