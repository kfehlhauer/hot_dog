use crate::backend::*;
use crate::components::*;
use dioxus::prelude::*;

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
            div { class: "page-not-found",
                p { "Page Not Found!"},
                p { "Check you path for typos! "},
                ul {
                    for item in segments.iter() {
                    li { "{item}" }
                }
            }
        }
    }
}

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        #[derive(serde::Deserialize)]
        struct DogApi {
            message: String,
        }

        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
    div { id: "dogview",
        img { src: img_src().unwrap_or_default() }
    }
    div { id: "buttons",
            button {
                id: "skip",
                onclick: move |_| img_src.restart(),
            "skip"
            }
            button {
                    id: "save",
                    onclick: move |_| async move {
                        // let current = img_src.cloned().unwrap();
                        img_src.restart();
                        let _ = save_dog(img_src().unwrap()).await;
                     },
                    "save!"
            }
        }
    }
}
