use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Header;

impl Component for Header {
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
                <div id="header">
                    <h1> { "KeyPress"  } </h1>
                </div>
            </>
        }
    }
}

