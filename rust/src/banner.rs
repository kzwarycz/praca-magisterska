use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

#[function_component(Banner)]
pub fn banner() -> Html {
    let scroll_to_gallery = Callback::from(|_| {
        if let Some(window) = window() {
            if let Some(element) = window.document().unwrap().get_element_by_id("gallery") {
                let element = element.dyn_into::<HtmlElement>().unwrap();
                element.scroll_into_view_with_bool(true);
            }
        }
    });

    html! {
        <section class="bg-gray-50">
            <div class="mx-auto max-w-screen-xl px-4 py-32 lg:flex lg:h-screen lg:items-center">
                <div class="mx-auto max-w-xl text-center">
                    <h1 class="text-3xl font-extrabold sm:text-5xl">
                        {"Web Apps with WebAssembly."}
                        <strong class="font-extrabold text-blue-700 sm:block"> {"Build components in Rust and Yew."} </strong>
                    </h1>

                    <p class="mt-4 sm:text-xl">
                        {"Leverage the safety and speed of Rust combined with the versatility of WebAssembly to deliver an unparalleled web experience."}
                    </p>

                    <button
                        class="mt-8 rounded bg-blue-500 px-6 py-2 text-white font-bold hover:bg-blue-700 focus:outline-none focus:ring"
                        onclick={scroll_to_gallery}
                    >
                        {"View Gallery"}
                    </button>
                </div>
            </div>
        </section>
    }
}
