use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use crate::state::{AppState, AppState2};
#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    name: String
}

mod state;

// 配置route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    // cfg.route("/health", web::get().to(health_check_handler));
    cfg.service(web::scope("/health")
        .route("", web::get().to(health_check_handler))
        .route("/student", web::post().to(student_handler))
    );
}
pub async fn student_handler(student: web::Json<Student>) -> impl Responder {
    println!("student = {:?}", student);
    
    HttpResponse::Ok().json("ok")
}
pub async fn health_check_handler(app_state: web::Data<AppState>,
                                  app_state2: web::Data<AppState2>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let health_check_response2 = &app_state2.health_check_response;
    let mut visit_count2 = app_state2.visit_count.lock().unwrap();
    let response = format!("{} {} times. {} {} times", health_check_response, visit_count
                           , health_check_response2, visit_count2);
    *visit_count += 1;
    *visit_count2 += 2;

    HttpResponse::Ok().json(&response)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm Ok.".to_string(),
        visit_count: Mutex::new(0),
    });

    let shared_data2 = web::Data::new(AppState2 {
        health_check_response: "I'm Ok2.".to_string(),
        visit_count: Mutex::new(0),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(shared_data2.clone())
            .configure(general_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
