use super::layout;
use super::primitives::{input, InputType};

use maud::{html, Markup};

pub fn index() -> Markup {
    layout(html! {
        h1 { "Welcome to the Invoice App" }
        p { "This is a simple invoice app built with Axum and SQLx" }
        div {
            (input("Email", "email", InputType::Email, None))
        }
    })
}
