extern crate lettre;

use std::env;

use actix_cors::Cors;
use actix_web::{http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use entity::feedback::ActiveModel;
use sea_orm::{entity::Set, ActiveModelTrait, Database, DbConn};
use serde::Deserialize;

use email::send_mail;
mod email;

#[derive(Deserialize, Debug)]
pub struct Feedback {
	r#type: String,
	comment: String,
	screenshot: String,
}

#[post("/feedbacks")]
async fn create_feedback(feedback: web::Json<Feedback>) -> impl Responder {
	let conn: DbConn = Database::connect(env::var("DATABASE_URL").unwrap())
		.await
		.unwrap();

	if feedback.r#type == "".to_string() {
		panic!("Feedback type is required");
	}
	if feedback.comment == "".to_string() {
		panic!("Feedback comment is required");
	}
	if feedback.screenshot != "".to_string()
		&& !feedback.screenshot.contains("data:image/png;base64")
	{
		panic!("Invalid feedback screenshot");
	}

	let new_feedback = ActiveModel {
		r#type: Set(feedback.r#type.to_owned()),
		comment: Set(feedback.comment.to_owned()),
		screenshot: Set(Some(feedback.screenshot.to_owned())),
		..Default::default()
	};

	if let Err(_) = send_mail(&feedback) {
		return HttpResponse::InternalServerError()
			.status(StatusCode::INTERNAL_SERVER_ERROR)
			.body("Internal server error");
	}

	new_feedback.insert(&conn).await.unwrap();

	HttpResponse::Ok()
		.status(StatusCode::CREATED)
		.body("Feedback created")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	HttpServer::new(|| {
		App::new()
			.wrap(
				Cors::default()
					.allow_any_origin()
					.allow_any_method()
					.allow_any_header(),
			)
			.service(create_feedback)
	})
	.bind(("127.0.0.1", 8000))?
	.run()
	.await
}
