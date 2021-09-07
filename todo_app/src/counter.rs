#![recursion_limit = "640"]
use yew::prelude::*;

pub struct Counter {
	// `ComponentLink` is like a reference to a component.
	// It can be used to send messages to the component
	link: ComponentLink<Self>,
	props: Properties,
}

#[derive(Clone, Debug, Properties)]
pub struct Properties {
	pub children: Children,
}

impl Component for Counter {
	type Message = ();
	type Properties = Properties;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, props }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		html! {
			<div style="color: blue">
				{self.props.children.clone()}
			</div>
		}
	}
}
