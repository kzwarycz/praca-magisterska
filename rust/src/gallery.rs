use crate::card::{Card, CardProps};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub cards: Vec<CardProps>,
}

#[function_component(Gallery)]
pub fn gallery(props: &GalleryProps) -> Html {
    html! {
        <div id="gallery" class="gallery grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-8 px-12">
            { for props.cards.iter().map(|card_props| html! {
                <Card url={card_props.url.clone()} title={card_props.title.clone()} id={card_props.id.clone()} />
            }) }
        </div>
    }
}
