use axum::{
    Json, Router,
    extract::Request,
    http::HeaderValue,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Payload {
    client_id: Uuid,
    email: String,
    is_active: bool,
    appointments: Vec<Appointment>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Appointment {
    id: u32,
    date: String,
    duration_minutes: u32,
    rate: f32,
}

#[derive(Serialize)]
struct ResponsePayload {
    client_id: Uuid,
    total_cost: f32,
}

async fn timing_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let mut response = next.run(req).await;
    let elapsed = format!("{:?}", start.elapsed());
    response
        .headers_mut()
        .insert("x-response-time", HeaderValue::from_str(&elapsed).unwrap());
    response
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn cpu_loop() -> String {
    let mut sum: u64 = 0;
    for i in 0..1_000_000 {
        sum = std::hint::black_box(sum + i);
    }
    sum.to_string()
}

async fn process(Json(payload): Json<Payload>) -> Json<ResponsePayload> {
    let total_cost = payload.appointments.iter().fold(0.0, |acc, app| {
        acc + (app.duration_minutes as f32 / 60.0) * app.rate
    });
    Json(ResponsePayload {
        client_id: payload.client_id,
        total_cost,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/loop", get(cpu_loop))
        .route("/process", post(process))
        .layer(middleware::from_fn(timing_middleware));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
