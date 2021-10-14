use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    limit: u8,
    #[serde(rename = "str1")]
    fizz_name: String,
    #[serde(rename = "str2")]
    buzz_name: String,
    #[serde(rename = "int1")]
    fizz_value: u8,
    #[serde(rename = "int2")]
    buzz_value: u8,
}

#[actix_web::get("/fizzbuzz")]
pub async fn fizzbuzz_api(params: actix_web::web::Query<Params>) -> impl actix_web::Responder {
    fizzbuzz(params.into_inner()).join(",")
}

fn fizzbuzz(params: Params) -> Vec<String> {
    (1..=params.limit)
        .map(|v| {
            let div_by_fizz = v % params.fizz_value == 0;
            let div_by_buzz = v % params.buzz_value == 0;
            match (div_by_fizz, div_by_buzz) {
                (true, true) => format!("{}{}", params.fizz_name, params.buzz_name),
                (true, false) => params.fizz_name.to_string(),
                (false, true) => params.buzz_name.to_string(),
                _ => format!("{}", v),
            }
        })
        .collect()
}
#[cfg(test)]
mod test {
    use super::*;
    fn to_string(val: &[&str]) -> Vec<String> {
        val.iter().map(|v| v.to_string()).collect()
    }
    #[test]
    fn basic_fizzbuzz() {
        assert_eq!(
            fizzbuzz(Params {
                limit: 20,
                fizz_name: "fizz".to_string(),
                fizz_value: 3,
                buzz_name: "buzz".to_string(),
                buzz_value: 5
            }),
            to_string(&[
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz",
                "13", "14", "fizzbuzz", "16", "17", "fizz", "19", "buzz"
            ])
        )
    }
    #[test]
    fn fizzbuzz_to_zero() {
        assert_eq!(
            fizzbuzz(Params {
                limit: 0,
                fizz_name: "fizz".to_string(),
                fizz_value: 3,
                buzz_name: "buzz".to_string(),
                buzz_value: 5
            }),
            to_string(&[])
        )
    }
}
