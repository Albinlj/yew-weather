// use stylist::yew::styled_component;
use yew::prelude::*;

pub enum Msg {
	AddOne,
}

pub struct App {
	value: i64,
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_: &yew::Context<Self>) -> Self {
		Self { value: 0 }
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::AddOne => {
				self.value += 1;
				true
			}
		}
	}

	fn view(&self, ctx: &yew::Context<App>) -> Html {
		let link = ctx.link();
		html! {
		    <div>
			<button onclick={link.callback(|_| Msg::AddOne)}>{ "+1"} </button>
			<p>{self.value} </p>
			<p>{ "hej" } </p>
		    </div>
		}
	}
}
