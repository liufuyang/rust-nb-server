#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate rust_nb;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rust_nb::{Feature, Model, ModelHashMapStore};
use std::sync::Mutex;

type ModelName = String;
type Outcome = String;

// We're going to store all of the messages here. No need for a DB.
type ModelInMem = Mutex<Model<ModelHashMapStore>>;

#[derive(Serialize, Deserialize)]
struct UpdateInput {
    updates: Vec<(Outcome, Vec<Feature>)>,
}

#[derive(Serialize, Deserialize)]
struct PredictInput {
    features: Vec<Vec<Feature>>,
}

#[put("/<model_name>", format = "json", data = "<input>")]
fn update(
    model_name: ModelName,
    input: Json<UpdateInput>,
    model_in_mem: State<ModelInMem>,
) -> JsonValue {
    let mut model = model_in_mem.lock().expect("map lock.");

    model.train(&model_name, &input.0.updates);

    json!({ "status": "ok" })
}

#[post("/<model_name>", format = "json", data = "<input>")]
fn predict(
    model_name: ModelName,
    input: Json<PredictInput>,
    model_in_mem: State<ModelInMem>,
) -> JsonValue {
    let model = model_in_mem.lock().expect("map lock.");

    let prediction = model.predict_batch(&model_name, &input.0.features);

    json!({ "predictions": prediction })
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(400)]
fn bad_request() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Bad request."
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/model", routes![update, predict])
        .register(catchers![not_found])
        .register(catchers![bad_request])
        .manage(Mutex::new(
            Model::new().with_pseudo_count(0.5).with_prior_factor(1.0),
        ))
        .launch();
}
