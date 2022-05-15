use serde::Serialize;
use wasm_bindgen::prelude::*;

use self::widget_form::FeedbackType;

mod close_button;
pub mod widget;
mod widget_form;

#[wasm_bindgen(module = "/index.js")]
extern "C" {
	#[wasm_bindgen]
	pub async fn take_screenshot() -> JsValue;
}

#[derive(Serialize)]
pub struct FeedbackRequest {
	r#type: FeedbackType,
	comment: String,
	screenshot: Option<String>,
}

impl FeedbackRequest {
	pub fn new(
		screenshot: Option<String>,
		r#type: FeedbackType,
		comment: String,
	) -> FeedbackRequest {
		FeedbackRequest {
			screenshot,
			r#type,
			comment,
		}
	}

	async fn send(&self, api: String) {
		let client = reqwest::Client::new();
		client.post(&api).json(&self).send().await.unwrap();
	}
}
