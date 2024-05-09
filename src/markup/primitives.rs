

//! The input primitive, renders an input field and a label with proper styling.

use maud::{html, Markup};

pub enum InputType {
    Text,
    Password,
    Email,
    Number,
}

pub fn input(label: &str, name: &str, input_type: InputType, placeholder: Option<&str>) -> Markup {

    let input_type = match input_type {
        InputType::Text => "text",
        InputType::Password => "password",
        InputType::Email => "email",
        InputType::Number => "number",
    };

    let placeholder = placeholder.unwrap_or("");

    html! {
        div {
        label for=(name) class="text-label dark:text-labeld" { (label) }
        }
        input id=(name) name=(name) type=(input_type) placeholder=(placeholder) class="bg-transparent dark:bg-inputd border rounded border-label dark:border-borderd focused:border-lila active:border-lila mt-1 px-4 py-2" {}
    }
}
