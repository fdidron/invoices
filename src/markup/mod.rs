pub mod home;
pub mod primitives;

use maud::{html, Markup};


pub fn layout(page: Markup) -> Markup {
    let nav = nav();
    html! {
        html lang="en" {
            head {
                title { "Invoices" }
                link rel="stylesheet" type="text/css" href="/static/app.css" {}
                script src="https://unpkg.com/htmx.org@1.9.12" integrity="sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2" crossorigin="anonymous" {}
                script src="https://unpkg.com/hyperscript.org@0.9.12" {}
                script src={"/static/main.js?ver=" (env!("CARGO_PKG_VERSION"))} {}
                link rel="preconnect" href="https://fonts.googleapis.com" {}
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin {}
                link href="https://fonts.googleapis.com/css2?family=League+Spartan:wght@500..700&display=swap" rel="stylesheet" {}
            }

            body class="bg-body dark:bg-darkbody text-typo dark:text-white font-sans font-medium" {
                div class="md:flex" {
                    (nav)
                    div {
                        (page)
                    }
                }
            }
        }
    }
}

fn nav() -> Markup {
    html! {
        nav class="bg-sidebar w-screen w-full h-[80px] md:h-screen md:min-h-[300px] md:w-[103px] md:rounded-r-[20px] flex md:flex-col justify-between" {
        div class="h-20 md:h-[103px] w-20 md:w-full bg-lila rounded-r-[20px] relative flex items-center justify-center" {
            div class="absolute bottom-0 left-0 right-0 top-[50%] bg-lilafade rounded-br-[20px] rounded-tl-[20px]" {
            }
            svg class="relative" xmlns="http://www.w3.org/2000/svg" width="28" height="26"{
                path fill="#FFF" fill-rule="evenodd" d="M20.513 0C24.965 2.309 28 6.91 28 12.21 28 19.826 21.732 26 14 26S0 19.826 0 12.21C0 6.91 3.035 2.309 7.487 0L14 12.9z" {}
            }
        }
        div class="flex md:block"{
            div class="w-20 md:w-full md:h-[100px] text-white flex items-center justify-center" {
                button class="text-bold" _="on click js app.toggleTheme()" {
                    "mode"
                }
            }
            div class="w-20 md:w-full md:h-[100px] text-white flex items-center justify-center border-t-[1px] border-[#494E6E]" {
                div class="rounded-[50%] bg-lila w-10 h-10 items-center justify-center flex" {
                        span class="text-bold" {
                            "?"
                        }
                }
            }
        }
    }
    }
}
