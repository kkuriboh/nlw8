use yew::{function_component, html, use_context, UseStateHandle};

use crate::icons::X;

#[function_component(CloseButton)]
pub fn close_button() -> Html {
    let form_state = use_context::<UseStateHandle<bool>>().expect("no context found");

    html! {
		<button onclick={move |_| form_state.set(!*form_state)} class={"top-5 right-5 absolute text-zinc-400 hover:text-zinc-100 group"}>
			<X id="x" class={"w-4 h-4 stroke-zinc-400 group-hover:stroke-zinc-100"}/>
		</button>
	}
}
