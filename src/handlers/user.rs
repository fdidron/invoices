use std::sync::Arc;

use axum::{extract::State, Form};
use axum_session_sqlx::SessionPgSession;
use maud::{html, Markup};
use sqlx::types::Uuid;
use validator::validate_email;

use crate::{models::user::get_or_create_user_by_email, AppState};

pub async fn signin(
    Form(email): Form<String>,
    session: SessionPgSession,
    State(data): State<Arc<AppState>>,
) -> Markup {

    let uid = session.get::<Uuid>("uid");
    if uid.is_some() {
        return html! {
            h1 { "You are already signed in" }
        };
    }
    if validate_email(email.clone()) {
        let user = get_or_create_user_by_email(&data.db, &email).await;

        if user.is_err() {
            return html! {
                h1 class="error" { "An error occurred while signing in" }
            };
        }

        let user = user.unwrap();
        let otp = user.get_otp();
        if otp.is_err() {
            return html! {
                h1 class="error" { "An error occurred while signing in" }
            };
        }

        let otp = otp.unwrap();
        session.set("uid", user.id);

        // TODO: Send the OTP to the user via email
        //
        dbg!(otp);

        return html! {
            h1 { "Check your email for the OTP" }
        };
    }
    // Failed to validate email
    html!(form method="post" action="/signin" {
        h1 class="error" { "Invalid email" }
        input type="email" name="email" placeholder="Email" required;
        input type="submit" value="Sign In";
    })
}
