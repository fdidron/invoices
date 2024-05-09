use maud::{Markup};

use crate::markup::home;

pub async fn index() -> Markup {
    home::index()
}
