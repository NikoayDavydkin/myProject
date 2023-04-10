mod bonfire_client;
mod graphql_schema;
mod serde;
mod synth;
mod tch_models;
mod update;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use std::convert::Infallible;
use std::env;
use tokio::sync::RwLock;
use warp::http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

use graphql_schema::{QueryRoot, Storage, SynthBenchData};
use update::update;

#[tokio::main]
async fn main() {
    let data = Storage::new(RwLock::new(SynthBenchData::new().await));
    let data_clone = data.clone();
    tokio::spawn(async move {
        update(data_clone).await;
    });
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(data)
        .finish();

    let port: u16 = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        _ => 19000,
    };

    println!("Playground: http://localhost:{}/graphql", port);

    let graphql_post = async_graphql_warp::graphql(schema)
        .and(warp::path("graphql"))
        .and_then(
            |(schema, request): (
                Schema<QueryRoot, EmptyMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        );

    let graphql_playground = warp::path("graphql").and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "content-type",
        ])
        .allow_methods(vec!["POST", "GET"]);

    let routes =
        graphql_playground
            .or(graphql_post)
            .with(cors)
            .recover(|err: Rejection| async move {
                if let Some(GraphQLBadRequest(err)) = err.find() {
                    return Ok::<_, Infallible>(warp::reply::with_status(
                        err.to_string(),
                        StatusCode::BAD_REQUEST,
                    ));
                }

                Ok(warp::reply::with_status(
                    "404 Not Found".to_string(),
                    StatusCode::NOT_FOUND,
                ))
            });

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
