#[macro_use]
extern crate rocket;

mod logic;
mod hook;

use crate::hook::{convert_json_circuit, JSONCircuit};
use crate::logic::Circuit;
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

#[post("/simulate", format = "json", data = "<json_circuit>")]

async fn simulate(json_circuit: Json<JSONCircuit>) -> Json<serde_json::Value> {
    let circuit = convert_json_circuit(json_circuit.into_inner());
    let v = circuit.get_state_vector();
    let b = circuit.get_basis_vectors();
    
    let v_serializable: Vec<(f64, f64)> = v.iter().map(|c| (c.re, c.im)).collect();
    let b_serializable: Vec<String> = b.iter().map(|s| s.clone()).collect();

    let response = serde_json::json!({
        "state_vector": v_serializable,
        "basis_vectors": b_serializable,
    });

    Json(serde_json::json!(response))
}

#[options("/simulate")]
fn options_simulate() -> &'static str {
    ""
}

#[launch]
fn rocket() -> _ {
    use rocket::http::Method;

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Post, Method::Get, Method::Options]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true);

    rocket::build()
        .mount("/", routes![simulate, options_simulate])
        .attach(cors.to_cors().expect("Error attaching CORS"))
}
