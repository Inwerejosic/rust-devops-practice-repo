mod database;
mod handlers;
mod model;
mod schema;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http};
use crate::database::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Initialize the SQLite Connection Pool via r2d2
    let pool = establish_connection();

    println!("🚀 Backend server is firing up...");
    println!("📡 API available at http://127.0.0.1:7070");

    HttpServer::new(move || {
        // 2. Configure CORS to allow your Vue dev environment
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // Standard Vue/Vite port
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION, 
                http::header::ACCEPT,
                http::header::CONTENT_TYPE
            ])
            .max_age(3600);

        // ... (rest of the file remains the same)

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            
            // --- API ROUTES ---

            // Health check
            .route("/health", web::get().to(handlers::health_check))

            // Authentication & Registration
            .route("/register", web::post().to(handlers::register_member))
            .route("/login", web::post().to(handlers::login))    
            
            // Member Management (Admin/User info)
            .route("/members", web::get().to(handlers::get_member))
            .route("/members/{id}", web::get().to(handlers::get_member_by_id))
            .route("/members/{id}", web::put().to(handlers::update_member))
            .route("/members/{id}", web::delete().to(handlers::delete_member))

            // Monthly Contributions
            .route("/contribute", web::post().to(handlers::record_contribution))
            .route("/contributions/{m_id}", web::get().to(handlers::get_member_contributions))
            
            // Admin Settings
            .route("/admin/fee", web::get().to(handlers::get_monthly_fee))
            .route("/admin/fee", web::put().to(handlers::set_monthly_fee))
    })
    .bind(("0.0.0.0", 7070))?
    .run()
    .await
}