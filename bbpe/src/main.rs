use iced::{widget, Element, Sandbox, Settings};

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    url: String,
}
#[derive(Debug, Clone)]
enum Message {
    Back,
    Forward,
    ChangeUrl(String),
    NavigateUrl,
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
        dbg!(message.clone());
        match message {
            Message::Back => todo!(),
            Message::Forward => todo!(),
            Message::ChangeUrl(url) => self.url = url,
            Message::NavigateUrl => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let control_bar = widget::Row::new()
            .push(widget::button("<").on_press(Message::Back))
            .push(widget::button(">").on_press(Message::Forward))
            .push(
                widget::text_input("Type a URL", &self.url, Message::ChangeUrl)
                    .on_submit(Message::NavigateUrl),
            );

        // main content
        widget::Column::new()
            .push(control_bar)
            .push(widget::text("Hello, world!"))
            .into()
    }
}
