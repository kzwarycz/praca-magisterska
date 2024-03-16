use crate::card::{Card, CardProps};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub cards: Vec<CardProps>,
}

#[function_component(Gallery)]
pub fn gallery(props: &GalleryProps) -> Html {
    html! {
        <div class="gallery">
            { for props.cards.iter().map(|card_props| html! {
                <Card url={card_props.url.clone()} title={card_props.title.clone()} description={card_props.description.clone()} />
            }) }
        </div>
    }
}