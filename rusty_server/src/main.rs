#[macro_use]
extern crate rocket;
extern crate lettre;

use std::env;

use entity::feedback::ActiveModel;
use rocket::serde::{json::Json, Deserialize};
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
	rocket::build().mount("/", routes![create_feedback])
}
