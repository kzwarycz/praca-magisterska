use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CardData {
    pub id: String,
    pub title: String,
    pub url: String,
}

impl Into<CardProps> for CardData {
    fn into(self) -> CardProps {
        CardProps {
            id: self.id,
            title: self.title,
            url: self.url,
        }
    }
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <a class="block">
            <img
                alt={props.title.clone()}
                src={props.url.clone()}
                class="w-full object-cover max-h-64 sm:max-h-80 lg:max-h-96"
            />
    
            <h3 class="mt-4 text-lg font-bold text-gray-900 sm:text-xl">{ &props.title }</h3>
    
            <p class="mt-2 max-w-sm text-gray-700">
                {"Lorem ipsum dolor sit amet consectetur, adipisicing elit. Magni reiciendis sequi ipsam incidunt."}
            </p>
        </a>
    }
    
}
