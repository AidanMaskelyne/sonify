use iced::{Application, Settings};
use sonify::app::SonifyApp;

fn main() -> iced::Result {
	return SonifyApp::run(Settings::default());
}
