use iced::{Sandbox, Element, Settings};
use iced::widget;

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    toggled: bool,
}
#[derive(Debug)]
enum Message {
    Toggle,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("bobabrowser")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Toggle => {
                self.toggled = !self.toggled;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        widget::Toggler::new(Some("hello".into()), self.toggled, |_| Message::Toggle).into()
    }
}
