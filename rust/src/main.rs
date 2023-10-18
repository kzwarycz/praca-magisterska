use yew::prelude::*;
use serde::Deserialize;
use anyhow::Error;
use reqwasm::http::Request;
use yew::ComponentLink;

#[derive(Deserialize, Clone, Debug)]
pub struct ImageData {
    url: String,
}

pub struct ImageComponent {
    images: Vec<ImageData>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ReceiveResponse(Result<Vec<ImageData>, Error>),
}

impl Component for ImageComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let request = Request::get("../../image_urls.json").send().unwrap();
        let callback = ctx.link().callback(
            move |response: reqwasm::http::Response<Result<Vec<ImageData>, Error>>| {
                Msg::ReceiveResponse(response.json().unwrap())
            },
        );
        request.fetch(callback);
        
        Self {
            images: Vec::new(),
            link: ctx.link().clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        self.images = data;
                    }
                    Err(error) => {
                        // Handle the error
                        log::error!("error: {:?}", error);
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="image-grid">
                { for self.images.iter().map(|image| html! {
                    <div>
                        <img src={image.url.clone()} alt="" />
                    </div>
                })}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<ImageComponent>();
}
