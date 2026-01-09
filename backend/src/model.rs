use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::member;
use crate::schema::{contributions, settings};
use chrono::NaiveDateTime; // Now correctly used below

// --- MEMBER MODELS ---

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = member)]
pub struct Member {
    pub id: i32,
    pub f_name: String,
    pub m_name: String,
    pub l_name: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub age: i32,
    pub is_admin: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = member)]
pub struct NewMember {
    pub f_name: String,
    pub m_name: String,
    pub l_name: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub age: i32,
    pub is_admin: bool,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = member)]
pub struct UpdateMember {
    pub f_name: Option<String>,
    pub m_name: Option<String>,
    pub l_name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub age: Option<i32>,
}

// --- CONTRIBUTION MODELS ---

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = contributions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Contribution {
    pub id: i32,
    pub member_id: i32,
    pub amount_paid: f64,
    pub month_period: String,
    pub created_at: Option<NaiveDateTime>, // Removed 'chrono::' prefix since it's imported
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = contributions)]
pub struct NewContribution {
    pub member_id: i32,
    pub amount_paid: f64,
    pub month_period: String,
}

// --- SETTINGS MODELS ---

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = settings)]
pub struct Setting {
    pub key: String,
    pub value: String,
}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// This matches what Rust sends back on success
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub member_id: i32,
    pub f_name: String,
    pub is_admin: bool,
}

// This is for the JWT token structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,    // User ID
    pub exp: usize,  // Expiration time
}
// src/model.rs

#[derive(Deserialize)]
pub struct NewMemberRequest {
    pub f_name: String,
    pub l_name: String,
    pub m_name: String,
    pub email: String,
    pub password: String, // This is the plain text password from the form
    pub address: String,
    pub age: i32,
    pub is_admin: bool,
}