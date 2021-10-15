use actix_web::{
    dev::{Body, ServiceResponse},
    test,
    web::{Bytes, Data},
    App,
};

async fn query(q: &str) -> ServiceResponse<Body> {
    let mut app = test::init_service(
        App::new()
            .data(fizzbuzz_server::model::State::default())
            .service(fizzbuzz_server::fizzbuzz_api)
            .service(fizzbuzz_server::stats_api),
    )
    .await;
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

#[actix_rt::test]
async fn stat_integration_test() {
    let state = Data::new(fizzbuzz_server::model::State::default());
    let mut app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(fizzbuzz_server::fizzbuzz_api)
            .service(fizzbuzz_server::stats_api),
    )
    .await;

    let req = test::TestRequest::get().uri("/stats").to_request();
    // first call to the stats endpoint should return a nb_hits = 0
    let result: serde_json::Value = test::read_response_json(&mut app, req).await;
    assert_eq!(
        result,
        serde_json::json! {{
            "fizzbuzz_params": null,
            "nb_hits": 0
        }}
    );
    // then we call once with fizz/buzz
    let req = test::TestRequest::get()
        .uri("/fizzbuzz?int1=3&int2=5&limit=9&str1=fizz&str2=buzz")
        .to_request();
    let _ = test::call_service(&mut app, req).await;

    // and twice with fuzz/bizz
    let req = test::TestRequest::get()
        .uri("/fizzbuzz?int1=3&int2=5&limit=9&str1=fuzz&str2=bizz")
        .to_request();
    let _ = test::call_service(&mut app, req).await;
    let req = test::TestRequest::get()
        .uri("/fizzbuzz?int1=3&int2=5&limit=9&str1=fuzz&str2=bizz")
        .to_request();
    let _ = test::call_service(&mut app, req).await;

    // the stats should give us fuzz/bizz
    let req = test::TestRequest::get().uri("/stats").to_request();
    let result: serde_json::Value = test::read_response_json(&mut app, req).await;
    assert_eq!(
        result,
        serde_json::json! {{
            "fizzbuzz_params": {
                "int1": 3,
                "int2": 5,
                "limit": 9,
                "str1": "fuzz",
                "str2": "bizz"
            },
            "nb_hits": 2
        }}
    );
}
