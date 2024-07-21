use iced::executor;
use iced::{Application, Command, Element, Theme};

#[derive(Debug)]
pub struct SonifyApp;

#[derive(Debug)]
pub enum Message {
	TogglePlayback,
}

impl Application for SonifyApp {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_flags: Self::Flags) -> (SonifyApp, Command<Message>) {
		return (
			SonifyApp,
			Command::none()
		);
	}

	fn title(&self) -> String {
		return String::from("Iced Music Player");
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		return Command::none();
	}

	fn view(&self) -> Element<Message> {
		return "Hello World!".into();
	}
}
