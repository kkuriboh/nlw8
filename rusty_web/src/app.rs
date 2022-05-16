use yew::prelude::*;

use crate::components::widget::Widget;

#[function_component(App)]
pub fn app() -> Html {
    let form_state = use_state(|| false);

	html! {
        <ContextProvider<UseStateHandle<bool>> context={form_state.clone()}>
			<Widget />
		</ContextProvider<UseStateHandle<bool>>>
	}
}
