use yew::prelude::*;
use yew::{classes};
#[function_component]

fn App() -> Html {
	let counter = use_state(|| 0);
	let onclick = {
		let counter = counter.clone();
		move |_| {
			let value = *counter + 1;
			counter.set(value);
		}
	};

	html! {
		<>
		<div class={classes!("main")}>
		<div class={classes!("card")}>
		<div class={classes!("card-header")}>
			<h3>{"Click To Add Count"}</h3>
		</div>
		<div class={classes!("card-body")}>
			<p>{ *counter }</p>
			<button class={classes!("btn")} {onclick}>{"Increase"}</button>
		</div>
		</div>
		</div>
		</>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}