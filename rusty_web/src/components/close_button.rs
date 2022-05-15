use yew::{function_component, html};

use crate::icons::X;

#[function_component(CloseButton)]
pub fn close_button() -> Html {
	html! {
		<button class={"top-5 right-5 absolute text-zinc-400 hover:text-zinc-100 group"}>
			<X id="x" class={"w-4 h-4 stroke-zinc-400 group-hover:stroke-zinc-100"}/>
		</button>
	}
}
