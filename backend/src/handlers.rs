use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::database::DbPool;
use crate::schema::{contributions, settings, member};
use crate::model::{
    Member, NewMember, UpdateMember, 
    Contribution, NewContribution, 
    Setting, 
    LoginRequest, AuthResponse, Claims, NewMemberRequest
};

// --- AUTH HANDLERS ---

pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<LoginRequest>
) -> impl Responder {
    let mut conn = pool.get().expect("Pool error");

    let user = member::table
        .filter(member::email.eq(&credentials.email))
        .first::<Member>(&mut conn);

    match user {
        Ok(m) => {
            let is_valid = verify(&credentials.password, &m.password).unwrap_or(false);

            if is_valid {
                let my_claims = Claims { sub: m.id, exp: 10000000000 };
                let token = encode(
                    &Header::default(), 
                    &my_claims, 
                    &EncodingKey::from_secret("secret".as_ref())
                ).unwrap();

                HttpResponse::Ok().json(AuthResponse {
                    token,
                    member_id: m.id,
                    f_name: m.f_name,
                })
            } else {
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("User not found"),
    }
}

pub async fn register_member(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewMemberRequest>
) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    // 1. Hash the password
    let hashed_password = match hash(&new_user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Error securing password"),
    };

    // 2. Map to Insertable Struct
    let member_to_save = NewMember {
        f_name: new_user.f_name.clone(),
        m_name: new_user.m_name.clone(),
        l_name: new_user.l_name.clone(),
        email: new_user.email.clone(),
        password: hashed_password,
        address: new_user.address.clone(),
        age: new_user.age,
    };

    // 3. Insert
    match diesel::insert_into(member::table)
        .values(&member_to_save)
        .execute(&mut conn) 
    {
        Ok(_) => HttpResponse::Created().body("Registration successful"),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Email might already be registered")
        }
    }
}

// --- MEMBER HANDLERS ---

pub async fn get_member(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    match member::table.load::<Member>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_member_by_id(
    pool: web::Data<DbPool>, 
    path_id: web::Path<i32>
) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    match member::table.find(*path_id).first::<Member>(&mut conn) {
        Ok(h) => HttpResponse::Ok().json(h),
        Err(_) => HttpResponse::NotFound().body(format!("Member with id {} not found", path_id)),
    }
}

pub async fn update_member(
    pool: web::Data<DbPool>,
    path_id: web::Path<i32>,
    member_update: web::Json<UpdateMember>,
) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    match diesel::update(member::table.find(*path_id))
        .set(&member_update.into_inner())
        .execute(&mut conn) {
            Ok(_) => {
                let updated = member::table
                    .find(*path_id)
                    .first::<Member>(&mut conn)
                    .expect("Failed to fetch updated member");
                HttpResponse::Ok().json(updated)
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
}

pub async fn delete_member(
    pool: web::Data<DbPool>, 
    path_id: web::Path<i32>
) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    match diesel::delete(member::table.find(*path_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Deleted Successfully"),
        Err(_) => HttpResponse::NotFound().body("Member not found"),
    }
}

// --- CONTRIBUTION HANDLERS ---

pub async fn record_contribution(
    pool: web::Data<DbPool>,
    payload: web::Json<NewContribution>
) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    let result = diesel::insert_into(contributions::table)
        .values(&payload.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json("Contribution recorded"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_member_contributions(
    pool: web::Data<DbPool>,
    m_id: web::Path<i32>
) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    let history = contributions::table
        .filter(contributions::member_id.eq(m_id.into_inner()))
        .load::<Contribution>(&mut conn);

    match history {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// --- SETTINGS HANDLERS ---

pub async fn get_monthly_fee(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    let result = settings::table
        .filter(settings::key.eq("monthly_fee"))
        .first::<Setting>(&mut conn);

    match result {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(_) => HttpResponse::NotFound().json("Fee setting not found"),
    }
}

pub async fn set_monthly_fee(
    pool: web::Data<DbPool>,
    new_fee: web::Json<String>
) -> impl Responder {
    let mut conn = pool.get().expect("Database pool error");

    match diesel::update(settings::table.filter(settings::key.eq("monthly_fee")))
        .set(settings::value.eq(new_fee.into_inner()))
        .execute(&mut conn) {
            Ok(_) => HttpResponse::Ok().json("Monthly fee updated"),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
}

pub async fn health_check(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get();
    match conn {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "available", "database": "connected"})),
        Err(_) => HttpResponse::ServiceUnavailable().json(serde_json::json!({"status": "unstable", "database": "disconnected"})),
    }
}