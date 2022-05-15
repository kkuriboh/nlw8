use app::App;

mod app;
mod components;
mod icons;

fn main() {
	dotenv::dotenv().ok();
	yew::start_app::<App>();
}
