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
                    <div class="row justify-content-start">
                        <div class="col-md-4">
                            <div class="row justify-content-start">
                                <div class="col-md-1">
                                    <img src="favicon.ico" class="app-logo"/>
                                </div>
                                <div class="col-md-1">
                                    { "KeyPress"  }
                                </div>
                                <div class="col-md-10"></div>
                            </div>
                        </div>
                        <div class="col-md-8"></div>
                    </div>
                </div>
            </>
        }
    }
}

