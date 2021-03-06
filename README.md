# fizzbuzz-server
## Exercise: Write a simple fizz-buzz REST server.

"The original fizz-buzz consists in writing all numbers from 1 to 100, and just replacing all multiples of 3 by ""fizz"", all multiples of 5 by ""buzz"", and all multiples of 15 by ""fizzbuzz"".
The output would look like this: ""1,2,fizz,4,buzz,fizz,7,8,fizz,buzz,11,fizz,13,14,fizzbuzz,16,...""."

Your goal is to implement a web server that will expose a REST API endpoint that:
- Accepts five parameters: three integers int1, int2 and limit, and two strings str1 and str2.
- Returns a list of strings with numbers from 1 to limit, where: all multiples of int1 are replaced by str1, all multiples of int2 are replaced by str2, all multiples of int1 and int2 are replaced by str1str2.

The server needs to be:
- Ready for production
- Easy to maintain by other developers

Bonus: add a statistics endpoint allowing users to know what the most frequent request has been. This endpoint should:
- Accept no parameter
- Return the parameters corresponding to the most used request, as well as the number of hits for this request

## Prerequisits

To build this server you either need Rust or Docker and `make`.

To install Rust you can call (you’ll need `curl`): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

All needed commands are listed in the Makefile. To see the list of all commands: `make`.

## Build

`make build`if you have rust, `make build_docker` to only build the docker.

## Run

`make run` or via docker `make run_docker_locally`.

## Use

The server runs at `localhost:8080`, you can query it with:

* `curl "http://localhost:8080/fizzbuzz?int1=3&int2=5&limit=20&str1=fizz&str2=buzz"`

you can also query the `/stats/` endpoint with:

`curl "http://localhost:8080/stats"`

## Test

run unit and integration tests with:

`make test`
