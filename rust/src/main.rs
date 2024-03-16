mod card;
mod gallery;

use gallery::Gallery;
use crate::card::CardProps;
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Gallery cards={
                vec![
                    CardProps {
                        url: "https://placehold.co/600x400".to_string(),
                        title: "Image 1".to_string(),
                        description: "This is the first image".to_string(),
                    },
                    CardProps {
                        url: "https://placehold.co/600x400".to_string(),
                        title: "Image 2".to_string(),
                        description: "This is the second image".to_string(),
                    },
                ]
            } />
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
