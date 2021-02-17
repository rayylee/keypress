use std::{
    time::Duration,
};
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::{ConsoleService, Task};
use wasm_bindgen::{JsCast, prelude::Closure};

use web_sys::{AudioBuffer, AudioContext, AudioDestinationNode};

use crate::common::msg::Msg;

const SOURCE_SOUND: &[u8] = include_bytes!("../content/sound/click.wav");

pub struct Body {
    link: ComponentLink<Self>,
    job: Option<Box<dyn Task>>,
    time: String,
    _standalone: (IntervalTask, IntervalTask),
}

impl Body {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from("Time: ") + &String::from(date.to_locale_time_string("en-US"))
    }
}

impl Component for Body {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let standalone_handle = IntervalService::spawn(
            Duration::from_secs(60),
            // This callback doesn't send any message to a scope
            Callback::from(|_| {
                ConsoleService::info("> Standalone timer callback.");
            }),
        );

        let clock_handle = IntervalService::spawn(
            Duration::from_secs(1),
            // Timer callback
            link.callback(|_| Msg::UpdateTime),
        );

        Self {
            link,
            job: None,
            time: Body::get_current_time(),
            _standalone: (standalone_handle, clock_handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonStart => {
                self.job = None;
                ConsoleService::info("> Button [Start] pressed.");

                let show_str = format!("> SOURCE_SOUND len is: {:?}", SOURCE_SOUND.len());
                ConsoleService::info(&show_str);

                let array_u8: js_sys::Uint8Array = js_sys::Uint8Array::from(SOURCE_SOUND);

                let array_buf: js_sys::ArrayBuffer = array_u8.buffer();

                let audio_ctx = AudioContext::new().unwrap();

                let song = audio_ctx.create_buffer_source().unwrap();
                let destination: AudioDestinationNode = audio_ctx.destination();

                let handler = move |buf: AudioBuffer| {
                    let buffer: Option<&AudioBuffer> = Some(&buf);

                    song.set_buffer(buffer);
                    song.connect_with_audio_node(destination.as_ref()).unwrap();
                    song.start().unwrap();
                };

                let handle: Box<dyn FnMut(_) + 'static> = Box::new(handler) as Box<dyn FnMut(_)>;

                let cb = Closure::wrap(handle);

                audio_ctx.decode_audio_data_with_success_callback(&array_buf,
                    cb.as_ref().unchecked_ref()).unwrap();

                cb.forget();

                true
            }
            Msg::UpdateTime => {
                self.time = Body::get_current_time();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="row justify-content-end">
                    <div class="col-sm-8"></div>
                    <div class="col-sm-4">
                        <button style="display:none" onclick=self.link.callback(|_| Msg::ButtonStart)>
                            { "Setting" }
                        </button>
                    </div>
                     <div class="col-sm-2">
                         <p> <font color="black"> { &self.time } </font></p>
                    </div>
                </div>
            </>
        }
    }
}
