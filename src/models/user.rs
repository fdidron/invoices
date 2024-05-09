use rand::{
    distributions::Alphanumeric,
    thread_rng, Rng,
};
use sqlx::{
    types::{chrono, Uuid},
    Pool, Postgres,
};
use totp_rs::{Algorithm, TOTP};
use anyhow::Error;

static TOTP_STEP: u64 = 15 * 60; // Login OTPS are valid for 15 minutes

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub profile_pic_url: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
    otp_secret: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn get_or_create_user_by_email(
    conn: &Pool<Postgres>,
    email: &str,
) -> Result<User, Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(conn)
        .await?;

    if user.id.is_nil() {
        // Create a random string for the OTP secret
        let secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        let user = sqlx::query_as!(
            User,
            "insert into users (email, otp_secret, name) values ($1, $2, '') returning *",
            email,
            secret
        )
        .fetch_one(conn)
        .await?;
        return Ok(user);
    }

    Ok(user)
}

impl User {
    pub fn get_otp(&self) -> Result<String, Error>{
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, TOTP_STEP, self.otp_secret.clone().into())?;
        let otp = totp.generate_current()?;
        Ok(otp)
    }
}
