use http::header::{HeaderName, HeaderValue};
use hyper::{
    service::service_fn, Body as HyperBody, Request as HyperRequest, Response as HyperResponse,
    Server,
};
use reqwest::{Client, Method, Request as ReqwestRequest, Response as ReqwestResponse};
use std::str::FromStr;
use std::{
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use tower::make::Shared;

async fn convert_hyper_to_reqwest_request(
    hyper_request: HyperRequest<hyper::Body>,
) -> Result<ReqwestRequest, reqwest::Error> {
    let method = Method::from_str(hyper_request.method().as_str()).unwrap();
    let uri = hyper_request.uri().to_string();
    let mut reqwest_builder = Client::new().request(method, &uri);

    for (name, value) in hyper_request.headers() {
        reqwest_builder = reqwest_builder.header(name.as_str(), value.to_str().unwrap());
    }

    let body_bytes = hyper::body::to_bytes(hyper_request.into_body())
        .await
        .unwrap();
    reqwest_builder = reqwest_builder.body(body_bytes);

    let reqwest_request = reqwest_builder.build()?;
    Ok(reqwest_request)
}

async fn convert_reqwest_to_hyper_response(
    reqwest_response: ReqwestResponse,
) -> Result<HyperResponse<HyperBody>, hyper::Error> {
    let status = reqwest_response.status();
    let reqwest_headers = reqwest_response.headers();

    let mut hyper_response_builder = HyperResponse::builder().status(status);

    for (name, value) in reqwest_headers.iter() {
        let header_name = HeaderName::from_bytes(name.as_str().as_bytes()).unwrap();
        let header_value = HeaderValue::from_str(value.to_str().unwrap()).unwrap();
        hyper_response_builder = hyper_response_builder.header(header_name, header_value);
    }

    let body_bytes = reqwest_response.bytes().await.unwrap();
    let hyper_body = HyperBody::from(body_bytes);

    let hyper_response = match hyper_response_builder.body(hyper_body) {
        Ok(res) => res,
        Err(e) => panic!("error: {}", e),
    };

    Ok(hyper_response)
}

async fn handle(req: HyperRequest<HyperBody>) -> Result<HyperResponse<HyperBody>, reqwest::Error> {
    let proxy_url = "<YOUR PROXY URL>";

    // Convert the Hyper request to a Reqwest request
    let reqwest_request = convert_hyper_to_reqwest_request(req).await.unwrap();

    // Create a reqwest client with the proxy
    let client_builder = Client::builder();

    let proxy = match reqwest::Proxy::all(proxy_url) {
        Ok(res) => res,
        Err(e) => panic!("error: {}", e),
    };

    let client = match client_builder.proxy(proxy).build() {
        Ok(result) => result,
        Err(e) => panic!("error {}", e),
    };

    println!("Request URL: {}", reqwest_request.url());
    println!("Request Method: {}", reqwest_request.method());
    println!(
        "Request timestamp: {:?}",
        SystemTime::now().duration_since(UNIX_EPOCH)
    );
    println!("Request headers: {:?}", reqwest_request.headers());

    let hyper_response = match client.execute(reqwest_request).await {
        Ok(res) => res,
        Err(error) => panic!("error: {}", error),
    };

    let response = match convert_reqwest_to_hyper_response(hyper_response).await {
        Ok(res) => res,
        Err(e) => panic!("error: {}", e)
    };

    println!("Response status: {}", response.status().as_str());
    println!("Response headers: {:?}", response.headers());

    Ok(response)
}

#[tokio::main]
async fn main() {
    let make_service = Shared::new(service_fn(handle));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Proxy server is listening on {}", addr);
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}
