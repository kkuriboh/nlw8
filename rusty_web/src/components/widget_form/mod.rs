use gloo::console::log;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

use crate::icons::{Bug, Camera, CircleNotch, Idea, Thought, Trash};

use super::{
	take_screenshot,
	widget_form::steps::{FeedbackContentStep, FeedbackSuccessStep, FeedbackTypeStep},
};

mod steps;

#[derive(PartialEq, Serialize, Clone)]
pub enum FeedbackType {
	Bug,
	Idea,
	Other,
}

/*impl ToString for FeedbackType {
	fn to_string(&self) -> String {
		match self {
			FeedbackType::Bug => "Bug",
			FeedbackType::Idea => "Idea",
			FeedbackType::Other => "Other",
		}
		.to_string()
	}
}*/

impl FeedbackType {
	pub fn title(&self) -> &'static str {
		match self {
			FeedbackType::Bug => "Problema",
			FeedbackType::Idea => "Ideia",
			FeedbackType::Other => "Outro",
		}
	}
	pub fn image(&self) -> Html {
		match self {
			FeedbackType::Bug => html!(<Bug class={"w-6 h-6"} />),
			FeedbackType::Idea => html!(<Idea class={"w-6 h-6"} />),
			FeedbackType::Other => html!(<Thought class={"w-6 h-6"} />),
		}
	}
}

#[function_component(WidgetForm)]
pub fn widget_form() -> Html {
	let feedback_type_state: UseStateHandle<Option<FeedbackType>> = use_state(|| None);
	let feedback_sent_state = use_state(|| false);
	let feedback_sent_state_clone = feedback_sent_state.clone();
	let feedback_type = FeedbackType::Bug;

	html! {
		<div class={"bg-zinc-900 p-4 relative rounded-2xl mb-4 flex flex-col items-center shadow-lg w-[calc(100vw-2rem)] md:w-auto"}>
			if *feedback_sent_state {
				<FeedbackSuccessStep />
			// TODO - change it to None later on
			} else if let Some(_) = &*feedback_type_state {
				<FeedbackTypeStep />
			} else {
				<FeedbackContentStep
					feedback_type={feedback_type}
					on_feedback_restart_requested={Callback::from(move |_| {
						feedback_sent_state_clone.set(false);
						feedback_type_state.set(None);
					})}
					on_feedback_sent={Callback::from(move |_| feedback_sent_state.set(true))}
				/>
			}
			<footer id="form_footer" class="text-xs text-neutral-400">
				{"Feito com ♥ pela "}
				<a class={"underline underline-offset-2"} href="https://rocketseat.com.br">{"Rocketseat"}</a>
			</footer>
		</div>
	}
}

#[function_component(Loading)]
fn loading() -> Html {
	html! {
		<div class={"w-6 h-6 flex items-center justify-center overflow-hidden"}>
			<CircleNotch class={"w-4 h-4 animate-spin"} />
		</div>
	}
}

#[derive(Properties, PartialEq)]
pub(super) struct ScrBtnProps {
    #[prop_or(None)]
	screenshot: Option<String>,
	on_screenshot_taken: Callback<Option<String>>,
}

#[function_component(ScreenshotButton)]
pub(super) fn screenshot_button(props: &ScrBtnProps) -> Html {
	let is_taking_screenshot = use_state(|| false);
	let is_taking_screenshot_clone = is_taking_screenshot.clone();
	let on_screenshot_taken = props.on_screenshot_taken.clone();
	let on_screenshot_taken_clone = props.on_screenshot_taken.clone();

	let handle_take_screenshot = move |_| {
		let on_screenshot_taken_clone = on_screenshot_taken_clone.clone();
		let is_taking_screenshot_clone = is_taking_screenshot_clone.clone();
		spawn_local(async move {
			is_taking_screenshot_clone.set(true);
			let screenshot = take_screenshot().await.as_string().unwrap_or_default();
			on_screenshot_taken_clone.emit(Some(screenshot.clone()));
			is_taking_screenshot_clone.set(false);
		});
	};

	if let Some(screenshot) = &props.screenshot {
		html! {
			<button
				type={"button"}
				onclick={move |_| on_screenshot_taken.emit(None)}
				style={format!("background-image: url({});", screenshot)}
				class={"p-1 w-10 h-10 rounded-md border-transparent flex justify-end items-end transition-colors group"}
			>
				<Trash class={"fill-zinc-400 group-hover:fill-zinc-100"}/>
			</button>
		}
	} else {
		html! {
			<button
				type={"button"}
				onclick={handle_take_screenshot}
				disabled={*is_taking_screenshot}
				class={"p-2 bg-zinc-800 rounded-md border-transparent hover:bg-zinc-700 transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-zinc-900 focus:ring-brand-500"}
			>
				if *is_taking_screenshot {
					<Loading />
				} else {
					<Camera class="w-6 h-6" />
				}
			</button>
		}
	}
}
