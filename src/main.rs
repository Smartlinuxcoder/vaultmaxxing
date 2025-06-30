mod routes;
use routes::home::Home;
use dioxus::{document, prelude::*};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        body { class: "min-h-screen bg-gradient-to-br from-[var(--ctp-base)] via-[var(--ctp-mantle)] to-[var(--ctp-crust)]" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav {
            class: "glass-strong p-3 shadow-lg sticky top-0 z-50",
            div {
                class: "container mx-auto flex justify-between items-center",
                div {
                    class: "flex items-center gap-2",
                    svg { 
                        height: "24px", 
                        view_box: "0 0 576 512", 
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "currentColor",
                        class: "text-[var(--ctp-text)]",
                        path { d: "M64 0C28.7 0 0 28.7 0 64L0 416c0 35.3 28.7 64 64 64l16 0 16 32 64 0 16-32 224 0 16 32 64 0 16-32 16 0c35.3 0 64-28.7 64-64l0-352c0-35.3-28.7-64-64-64L64 0zM224 320a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm0-240a160 160 0 1 1 0 320 160 160 0 1 1 0-320zM480 221.3L480 336c0 8.8-7.2 16-16 16s-16-7.2-16-16l0-114.7c-18.6-6.6-32-24.4-32-45.3c0-26.5 21.5-48 48-48s48 21.5 48 48c0 20.9-13.4 38.7-32 45.3z" }
                    }
                    h1 {
                        class: "text-xl font-bold text-[var(--ctp-text)]",
                        "VaultMaxxing"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
