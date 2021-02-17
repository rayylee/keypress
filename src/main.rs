// From https://www.syntaxsuccess.com/viewarticle/experimenting-with-rust-and-webassembly
#![recursion_limit = "512"]

mod components;
mod common;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::agent::{Dispatched};
use web_sys::{KeyboardEvent};
use wasm_bindgen::{JsCast, prelude::Closure};

use crate::components::{
    header::Header,
    footer::Footer,
    body::Body,
    keyboard::Keyboard,
};
use crate::common::event_bus::{EventBus, Request};

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header>
                    <Header/>
                </header>
                <div>
                    <Body/>
                    <Keyboard/>
                </div>
                <footer>
                    <Footer/>
                </footer>
            </>
        }
    }
}

fn main() {
    let mut event_bus = EventBus::dispatcher();
    let window = web_sys::window().unwrap();

    let handler_submit = move | e: KeyboardEvent | {
        e.stop_propagation();
        // link.callback(move | e: KeyboardEvent | Key::SetText(e.key()));
        // event_bus.send(Request::EventBusMsg("Message received".to_owned()));
        event_bus.send(Request::EventBusMsg(e.key()));
    };

    let handler = Box::new(handler_submit) as Box<dyn FnMut(_)>;

    let cb = Closure::wrap(handler);

    window.add_event_listener_with_callback("keydown",
        cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    yew::start_app::<Model>();
}
