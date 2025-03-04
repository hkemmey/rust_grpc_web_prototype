use warp::Filter;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Deserialize)]
struct FormData {
    message: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

 //   let request = tonic::Request::new(HelloRequest {
   //     name: "Tonic".into(),
   // });

    //let response = client.say_hello(request).await?;

    //println!("RESPONSE={:?}", response);

    //Ok(())

    let grpc_client = warp::any().map(|| async {
        GreeterClient::connect("http://[::1]:50051").await.unwrap()
    });

    // API Route: Accept form data and call grpc server
    let api_route = warp::post()
        .and(warp::path("submit"))
        .and(warp::body::json())
        .and(grpc_client.clone())
        .and_then(handle_form_submission);

    // Serve Static files (react front end)
    let static_files = warp::fs::dir("../../frontend/dist");

    // Combine routes
    let routes = api_route.or(static_files);

    println!("Server running on localhost:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}

async fn handle_form_submission(
    form: FormData,
    client: impl std::future::Future<Output = GreeterClient<Channel>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut client = client.await;
    let request = tonic::Request::new(HelloRequest {
        name: form.message
    });

    match client.say_hello(request).await {
        Ok(response) => {
            let response_message = response.into_inner().message;
            Ok(warp::reply::json(&ApiResponse { message: response_message }))
        }
        Err(_) => Err(warp::reject::not_found()),
    }
}
