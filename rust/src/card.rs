use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    pub url: String,
    pub title: String,
    pub description: String,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="max-w-sm rounded overflow-hidden shadow-lg">
            <img class="w-full" src={props.url.clone()} alt={props.title.clone()} />
            <div class="px-6 py-4">
                <div class="font-bold text-xl mb-2">{ &props.title }</div>
                <p class="text-gray-700 text-base">
                    { &props.description }
                </p>
            </div>
        </div>
    }
}
