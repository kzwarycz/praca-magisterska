mod card;
mod gallery;

use crate::card::CardProps;
use gallery::Gallery;
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
                let fetched_images: Vec<ImageData> = Request::get("/static/image_data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                let card_props: Vec<CardProps> = fetched_images.into_iter().map(|img| CardProps {
                    id: img.id,
                    title: img.title,
                    url: img.url,
                }).collect();
                images.set(card_props);
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
    yew::Renderer::<App>::new().render();
}
