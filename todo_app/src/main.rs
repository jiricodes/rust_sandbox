// Recursion limit because of html macro in view function, which uses a lot of recusrion to process the syntax
// recursion limit reached when packaging the program
#![recursion_limit = "640"]
use yew::prelude::*;

mod counter;
use counter::Counter;

enum Msg {
	AddOne,
	SubOne,
}

struct Model {
	// `ComponentLink` is like a reference to a component.
	// It can be used to send messages to the component
	link: ComponentLink<Self>,
	value: i64,
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, value: 0 }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::AddOne => {
				self.value += 1;
				// the value has changed so we need to
				// re-render for it to appear on the page
				true
			}
			Msg::SubOne => {
				self.value -= 1;
				true
			}
		}
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		html! {
			<div>
				<button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
				<button onclick=self.link.callback(|_| Msg::SubOne)>{ "-1" }</button>
				<Counter>
					<p>{"HERE: "}{self.value}</p>
				</Counter>
			</div>
		}
	}
}

fn main() {
	yew::start_app::<Model>();
}
