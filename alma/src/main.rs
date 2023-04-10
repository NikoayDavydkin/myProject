mod bonfire_client;
mod graphql_schema;
mod parse;
mod parse_dispatcher;
mod parsers;
mod serde;
mod update;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use std::convert::Infallible;
use tokio::sync::RwLock;
use warp::http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

use graphql_schema::{AlmaData, MutationRoot, QueryRoot, Storage};
use update::update;

#[tokio::main]
async fn main() {
    let data = Storage::new(RwLock::new(AlmaData::new().await));
    let data_clone = data.clone();
    tokio::spawn(async move {
        update(data_clone).await;
    });
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(data)
        .finish();

    println!("Playground: http://localhost:11000/graphql");

    let graphql_post = async_graphql_warp::graphql(schema)
        .and(warp::path("graphql"))
        .and_then(
            |(schema, request): (
                Schema<QueryRoot, MutationRoot, EmptySubscription>,
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

    warp::serve(routes).run(([0, 0, 0, 0], 11000)).await;
}
