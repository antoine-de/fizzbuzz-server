use actix_web::{
    dev::{Body, ServiceResponse},
    test,
    web::Bytes,
    App,
};

async fn query(q: &str) -> ServiceResponse<Body> {
    let mut app = test::init_service(App::new().service(fizzbuzz_server::fizzbuzz_api)).await;
    let req = test::TestRequest::get().uri(q).to_request();
    test::call_service(&mut app, req).await
}

#[actix_rt::test]
async fn basic_fizzbuzz_integration_test() {
    let resp = query("/fizzbuzz?int1=3&int2=5&limit=20&str1=fizz&str2=buzz").await;
    assert!(resp.status().is_success());

    let result = test::read_body(resp).await;
    assert_eq!(
        result,
        Bytes::from_static(
            b"1,2,fizz,4,buzz,fizz,7,8,fizz,buzz,11,fizz,13,14,fizzbuzz,16,17,fizz,19,buzz"
        )
    );
}

// setting a limit to zero should return an empty string
#[actix_rt::test]
async fn zero_limit_fizzbuzz_integration_test() {
    let resp = query("/fizzbuzz?int1=3&int2=5&limit=0&str1=fizz&str2=buzz").await;
    assert!(resp.status().is_success());

    let result = test::read_body(resp).await;
    assert_eq!(result, Bytes::from_static(b""));
}

// setting a negative limit is an error
#[actix_rt::test]
async fn negative_limit_fizzbuzz_integration_test() {
    let resp = query("/fizzbuzz?int1=3&int2=5&limit=-20&str1=fizz&str2=buzz").await;
    assert!(resp.status().is_client_error());
}

// setting a too big limit is an error (the limit is arbitrary 256 to limit memory attacks)
#[actix_rt::test]
async fn big_limit_fizzbuzz_integration_test() {
    let resp = query("/fizzbuzz?int1=3&int2=5&limit=9999999&str1=fizz&str2=buzz").await;
    assert!(resp.status().is_client_error());
}
