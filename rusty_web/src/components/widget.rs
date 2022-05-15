use yew::prelude::*;

use super::widget_form::WidgetForm;
use crate::icons::ChatTeardropDots;

#[function_component(Widget)]
pub fn widget() -> Html {
	let form_state = use_state(|| false);
	let form_state_copy = form_state.clone();

	html! {
		<div class={"absolute bottom-4 right-5 md:bottom-8 md:right-8 flex flex-col items-end"}>
			if *form_state_copy {
				<WidgetForm />
			}
			<button id="widget_btn" class={"h-12 bg-brand-500 text-white px-3 rounded-full flex items-center group"} onclick={move |_| form_state.set(!*form_state)}>
					<ChatTeardropDots class={"w-6 h-6"} />
				<span class={"max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-500 ease-linear"}>
					<span class={"pl-2"}/>
					{"feedback"}
				</span>
			</button>
		</div>
	}
}
