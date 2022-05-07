use yew::prelude::*;

#[function_component(Widget)]
pub fn widget() -> Html {
    let hover_state = use_state(|| false);
    let hover_state_clone = hover_state.clone();

    html! {
        <div class="absolute bottom-4 right-5 md:bottom-8 md:right-8 flex flex-col items-end"
            onmouseover={Callback::from(move |_| hover_state.set(!*hover_state))}
        >
            <button class="h-12 bg-brand-500 text-white px-3 rounded-full flex items-center">
                <ChatTeardropDots />
                if *hover_state_clone {
                    <span className="max-w-0 overflow-hidden hover:max-w-xs ">
                        {"feedback"}
                    </span>
                }
            </button>
        </div>
    }
}

#[function_component(ChatTeardropDots)]
fn chat_teardrop_dots() -> Html {
    html! {
        <svg class="fill-zinc-100 w-6 h-6" xmlns="http://www.w3.org/2000/svg" width="192" height="192" fill="#f4f4f5" viewBox="0 0 256 256"><rect width="256" height="256" fill="none"></rect><path d="M132,216H47.7a7.6,7.6,0,0,1-7.7-7.7V124a92,92,0,0,1,92-92h0a92,92,0,0,1,92,92h0A92,92,0,0,1,132,216Z" fill="none" stroke="#f4f4f5" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"></path><circle cx="132" cy="128" r="12"></circle><circle cx="84" cy="128" r="12"></circle><circle cx="180" cy="128" r="12"></circle></svg>
    }
}
