mod card;
mod gallery;

use crate::card::CardProps;
use gallery::Gallery;
use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_logger;
use log::{Level, info};
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct ImageData {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[function_component(App)]
fn app() -> Html {
    let images = use_state(|| vec![]);
    {
        let images = images.clone();
        use_effect_with((), move |_| {
            let images = images.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/image_data.json").send().await {
                    Ok(response) => {
                        let text = response.text().await.unwrap_or_else(|_| "Failed to read response text".to_string());
                        web_sys::console::log_1(&format!("Response text: {}", text).into());
                        match serde_json::from_str::<Vec<ImageData>>(&text) {
                            Ok(fetched_images) => {
                                let card_props: Vec<CardProps> = fetched_images.into_iter().map(|img| CardProps {
                                    id: img.id,
                                    title: img.title,
                                    url: img.url,
                                }).collect();
                                images.set(card_props);
                            },
                            Err(e) => web_sys::console::log_1(&format!("Failed to deserialize: {:?}", e).into()),
                        }
                    },
                    Err(e) => web_sys::console::log_1(&format!("Failed to fetch: {:?}", e).into()),
                }
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{ "Image Gallery" }</h1>
            <div>
                <h3>{"Images to view"}</h3>
                <Gallery cards={(*images).clone()} />
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Debug));
    info!("Starting the Yew app");
    yew::Renderer::<App>::new().render();
}
