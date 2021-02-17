use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::{ConsoleService};
use web_sys::{KeyboardEvent};

use crate::common::msg::Key;

pub struct Input {
    link: ComponentLink<Self>,
}

impl Component for Input {
    type Message = Key;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
	match msg {
	    Key::SetText(text) => {
                let s = format!("> key {} pressed.", text);
                ConsoleService::info(s.as_str());
	    }
	    Key::SelectLevel(level) => {
                let msg = format!("> input level [{:?}].", level);
                ConsoleService::info(&msg);
            }
	    Key::SelectChapter(chaper) => {
                let msg = format!("> input chaper: {}.", chaper);
                ConsoleService::info(&msg);
            }
	    Key::WordNextPre(text) => {
                let s = format!("> word {}.", text);
                ConsoleService::info(s.as_str());
            }
	    Key::Submit => {
                ConsoleService::info("> key [enter] pressed.");
	    }
	}

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

     fn view(&self) -> Html {
         html! {
             <input
                 onkeydown=self.link.batch_callback(move |e: KeyboardEvent| {
                     e.stop_propagation();
                     if e.key() == "Enter" { vec![Key::Submit] } else { vec![Key::SetText(e.key())] }
                 })
             />
         }
     }
}

