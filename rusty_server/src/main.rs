#[macro_use]
extern crate rocket;
extern crate lettre;

use std::env;

use entity::feedback::ActiveModel;
use rocket::{
	fairing::{Fairing, Info, Kind},
	http::Header,
	serde::{json::Json, Deserialize},
	Request, Response,
};
use sea_orm::{entity::Set, ActiveModelTrait, Database, DbConn};

use email::send_mail;
mod email;

#[derive(Deserialize)]
pub struct Feedback {
	r#type: String,
	comment: String,
	screenshot: Option<String>,
}

#[post("/feedbacks", data = "<feedback>")]
async fn create_feedback(feedback: Json<Feedback>) -> rocket::http::Status {
	let conn: DbConn = Database::connect(env::var("DATABASE_URL").unwrap())
		.await
		.unwrap();

    if feedback.r#type == "".to_string() {
        panic!("Feedback type is required");
    }
    if feedback.comment == "".to_string() {
        panic!("Feedback comment is required");
    }
    if feedback.screenshot.is_none() || !feedback.screenshot.as_ref().unwrap().contains("data:image/png;base64") {
        panic!("Invalid feedback screenshot");
    }

	let new_feedback = ActiveModel {
		r#type: Set(feedback.r#type.to_owned()),
		comment: Set(feedback.comment.to_owned()),
		screenshot: Set(feedback.screenshot.to_owned()),
		..Default::default()
	};

	send_mail(&feedback);

	new_feedback.insert(&conn).await.unwrap();
	rocket::http::Status::Created
}

#[launch]
fn rocket() -> _ {
	dotenv::dotenv().ok();
	rocket::build()
		.mount("/", routes![create_feedback])
        .attach(CORS)
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
	fn info(&self) -> Info {
		Info {
			name: "Add CORS headers to responses",
			kind: Kind::Response,
		}
	}

	async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
		response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8080"));
		response.set_header(Header::new(
			"Access-Control-Allow-Methods",
			"POST, GET, PATCH, OPTIONS",
		));
		response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
		response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
	}
}
