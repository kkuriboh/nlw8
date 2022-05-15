use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::{function_component, html, html::onchange::Event, use_state, Callback, Properties};

use crate::components::{
	close_button::CloseButton,
	widget_form::{Loading, ScreenshotButton},
	FeedbackRequest,
};
use crate::icons::ArrowLeft;

use super::FeedbackType;

#[function_component(FeedbackTypeStep)]
pub fn feedback_type_step() -> Html {
	html! {
		<h1>{"Feedback Type"}</h1>
	}
}

#[derive(Properties, PartialEq)]
pub struct ContentProps {
	pub feedback_type: FeedbackType,
	pub on_feedback_restart_requested: Callback<()>,
	pub on_feedback_sent: Callback<()>,
}

#[function_component(FeedbackContentStep)]
pub fn feedback_content_step(props: &ContentProps) -> Html {
	let screenshot = use_state(|| Option::<String>::None);
    let screenshot_clone = screenshot.clone();
	let comment = use_state(|| String::new());
	let comment_clone = comment.clone();
	let is_sending = use_state(|| false);

	let on_feedback_restart_requested = props.on_feedback_restart_requested.clone();

	// TOP 10 CURSED CODE IMAGES
	let comment_handler = comment.clone();
	let screenshot_handler = screenshot.clone();
	let is_sending_handler = is_sending.clone();
	let feedback_type = props.feedback_type.clone();
	let handle_submit = move |e: html::onsubmit::Event| {
		e.prevent_default();
		if comment_handler.is_empty() && screenshot_handler.is_none() {
			return;
		}
		let is_sending_handler = is_sending_handler.clone();
		let screenshot_handler = screenshot_handler.clone();
		let comment_handler = comment_handler.clone();
		let feedback_type = feedback_type.clone();
		spawn_local(async move {
			is_sending_handler.set(true);
			let feedback = FeedbackRequest::new(
				screenshot_handler
					.as_ref()
					.unwrap_or(&String::new())
					.to_string(),
				feedback_type,
				comment_handler.to_string(),
			);
			#[cfg(not(debug_assertions))]
			feedback.send(std::env!("API_URL")).await;
			#[cfg(debug_assertions)]
			feedback
				.send("http://localhost:8000/feedbacks".to_string())
				.await;
			is_sending_handler.set(false);
		})
	};

	html! {
		<>
			<header id="widget_content_header">
				<button class={"top-5 left-5 absolute group"} type="button" id="arrow_btn" onclick={move |_| on_feedback_restart_requested.emit(())}>
					<ArrowLeft class={"w-4 h-4 stroke-zinc-400 group-hover:stroke-zinc-100"}/>
				</button>
				<span class={"text-xl leading-6 flex items-center gap-2"}>
					{props.feedback_type.image()}
					{props.feedback_type.title()}
				</span>
				<CloseButton />
			</header>
			<form onsubmit={handle_submit} id="widget_content_form" class={"my-4 w-full"}>
				<textarea
					onchange={move |e: Event| comment.set(e.target().unwrap().unchecked_into::<HtmlTextAreaElement>().value())}
					placeholder="Conte com detalhes o que está acontecendo..."
					class={"min-w-[304px] w-full min-h-[112px] text-sm placeholder-zinc-400 text-zinc-100 border-zinc-600 bg-transparent rounded-md focus:border-brand-500 focus:ring-brand-500 focus:ring-1 focus:outline-none resize-none scrollbar-thumb-zinc-700 scrollbar-track-transparent scrollbar-thin"}
				/>
				<footer class={"flex gap-2 mt-2"}>
					<ScreenshotButton
                        screenshot={
                            if let Some(screenshot) = screenshot_clone.as_ref() {
                                Some(screenshot.clone())
                            } else {
                                None
                            }
                        }
                        on_screenshot_taken={Callback::from(move |val: Option<String>| screenshot.set(val))}
                    />
					<button
						type="submit"
						disabled={comment_clone.len() == 0 || *is_sending}
						class={"p-2 bg-brand-500 rounded-md border-transparent flex-1 flex justify-center items-center text-sm hover:bg-brand-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-zinc-900 focus:ring-brand-500 transition-colors disabled:opacity-50 disabled:hover:bg-brand-500"}
					>
						if *is_sending {
							<Loading />
						} else {
							{"Enviar feedback"}
						}
					</button>
				</footer>
			</form>
		</>
	}
}

#[function_component(FeedbackSuccessStep)]
pub fn feedback_success_step() -> Html {
	html! {
		<h1>{"Feedback Success"}</h1>
	}
}
