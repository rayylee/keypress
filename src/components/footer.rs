use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Footer;

impl Component for Footer {
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
                <div id="footer">
		    { "Powered by " }
		    <a href="https://yew.rs">{ "Yew" }</a>
		    { " using " }
		    <a href="https://www.rust-lang.org">{ "Rust" }</a>
		    { " and crates from " }
		    <a href="https://crates.io">{ "Crates" }</a>
                </div>
            </>
        }
    }
}
