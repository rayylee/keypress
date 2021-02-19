use std::{
    time::Duration,
};
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::{ConsoleService};

use crate::common::msg::Msg;

pub struct Body {
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
            time: Body::get_current_time(),
            _standalone: (standalone_handle, clock_handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                <div class="container">
                    <div class="row justify-content-end">
                        <div class="col-md-10"></div>
                        <div class="col-md-2">
                            <p> <font color="black"> { &self.time } </font></p>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
