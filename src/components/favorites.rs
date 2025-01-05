use crate::backend::*;
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let mut favorites = use_resource(crate::backend::list_dogs);

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                    for (id, url) in favorites.suspend()?.cloned().unwrap() {
                        div {
                            key: id,
                            class: "favorite-dog",
                            img { src: "{url}"}
                            button {id: "delete",
                                onclick: move |_| {
                                    async move {
                                        delete_dog(id).await.unwrap();
                                        favorites.restart();
                                    }
                                },
                            "❌" }
                        }
                    }
            }
        }
    }
}
