// use Axum framework to make an API
//
// define route structure:
//      /       http-GET
//      /classes
//      /classes http-GET
//      /classes http-POST

// Import libs
use axum::{
    routing::{get,post},
    response::IntoResponse,
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize}; // these have to be added to Cargo.toml


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init(); // keep track of studd

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // give the (endpoints = routes)
        // must have a root route. 
        .route("/", get(root)) // passing the functions that will be used as action handler. Wrap those functions around HTTPS action
        // `POST /users` goes to `create_user`
        .route("/classes", get(get_classes)) // so that router knows what to do with an http request

        .route("/classes", post(create_class)); 

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // pass tcp lister to axum framework
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!" // last statemet = return value
}

async fn create_class(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    // accept Json payload
    Json(payload): Json<CreateClass>,
    // return type = http status code. Hopefully 200  
) -> (StatusCode, Json<Class>) {
    let class = Class {
        crn: 3550, // where is this value coming from
        name: payload.name, // read in from the payload the value of the same?
    };
    (StatusCode::CREATED, Json(class)) // pass Json format of class struct
}

// derive is an annotation that takes the desirialized struct from the serd package and add additional fields to our struct
#[derive(Deserialize)] // change this to whatever endpoint you have
struct CreateClass {
    // Create class for the payload
    name: String,
    // actual struct is the class struct
}

#[derive(Serialize)]
struct Class {
    crn: u64,
    name: String,
    // add more parameters. ex: description
}

// function to handle the get classes
async fn get_classes() -> &'static str {
    "Class list" 
}
