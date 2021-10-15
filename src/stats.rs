use crate::model::{FizzBuzzParams, State};
use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatsResults {
    nb_hits: usize,
    fizzbuzz_params: Option<FizzBuzzParams>,
}

#[actix_web::get("/stats")]
pub async fn stats_api(state: actix_web::web::Data<State>) -> Json<StatsResults> {
    state
        .most_used_query()
        .map(|(p, nb_hits)| {
            Json(StatsResults {
                fizzbuzz_params: Some(p),
                nb_hits,
            })
        })
        .unwrap_or_else(|| {
            Json(StatsResults {
                fizzbuzz_params: None,
                nb_hits: 0,
            })
        })
}
