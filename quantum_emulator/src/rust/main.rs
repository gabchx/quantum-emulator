// main.rs
#[macro_use]
extern crate rocket;

mod hook;
mod logic;

use crate::hook::{convert_json_circuit, JSONCircuit};
use crate::logic::bloch_sphere_angles_per_qubit;
use rocket::serde::json::Json;
use rocket::{fs::NamedFile, response::Redirect};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::env;

#[post("/simulate", format = "json", data = "<json_circuit>")]
async fn simulate(json_circuit: Json<JSONCircuit>) -> Json<serde_json::Value> {
    let circuit = convert_json_circuit(json_circuit.into_inner());
    let v = circuit.get_state_vector();
    let b = circuit.get_basis_vectors();
    let p = circuit.get_state_probabilities();

    let v_serializable: Vec<(f64, f64)> = v.iter().map(|c| (c.re, c.im)).collect();
    let b_serializable: Vec<String> = b.iter().cloned().collect();

    let bloch_angles = bloch_sphere_angles_per_qubit(&v, circuit.n_qubits);

    let response = serde_json::json!({
        "state_vector": v_serializable,
        "basis_vectors": b_serializable,
        "probabilities": p,
        "bloch_angles": bloch_angles,
    });

    Json(response)
}

#[options("/simulate")]
fn options_simulate() -> &'static str {
    ""
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/home"))
}

#[get("/home")]
async fn home() -> Result<NamedFile, std::io::Error> {
    let mut path = env::current_dir().expect("Failed to get current directory");
    path.push("src/front/index.html");
    NamedFile::open(path).await
}

#[launch]
fn rocket() -> _ {
    use rocket::http::Method;

    // Option A: Specify both origins
    // let allowed_origins = AllowedOrigins::some(
    //     &["http://localhost:8000", "http://127.0.0.1:8000"],
    //     &[] as &[&str],
    // );

    // Option B: Allow all origins
    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: false,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS");

    rocket::build()
        .mount("/", routes![simulate, options_simulate, index, home])
        .attach(cors)
}
