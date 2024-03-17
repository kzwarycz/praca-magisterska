mod card;
mod gallery;
mod banner;
mod footer;

use crate::card::CardProps;
use gallery::Gallery;
use banner::Banner;
use footer::Footer;
use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

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
                        match serde_json::from_str::<Vec<ImageData>>(&text) {
                            Ok(fetched_images) => {
                                let card_props: Vec<CardProps> = fetched_images.into_iter().map(|img| CardProps {
                                    id: img.id,
                                    title: img.title,
                                    url: img.url,
                                }).collect();
                                images.set(card_props);
                            },
                            Err(_e) => {
                            },
                        }
                    },
                    Err(_e) => {
                    },
                }
            });
            || ()
        });
    }

    html! {
        <>
            <Banner />
                <div>
                    <Gallery cards={(*images).clone()} />
                </div>
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
