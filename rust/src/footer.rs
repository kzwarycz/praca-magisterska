use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="text-center py-4">
        <hr class="my-12 h-0.5 border-t-0 bg-neutral-100" />
            {"Kamil Zwarycz. Master's Thesis 2024"}
        </footer>
    }
}
