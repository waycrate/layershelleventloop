use iced::widget::{button, column, row, text, text_input};
use iced::{event, Element, Event, Length, Task as Command, Theme};
use iced_layershell::actions::LayershellCustomActions;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::Application;

pub fn main() -> Result<(), iced_layershell::Error> {
    let args: Vec<String> = std::env::args().collect();

    let mut binded_output_name = None;
    if args.len() >= 2 {
        binded_output_name = Some(args[1].to_string())
    }

    Counter::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 400)),
            exclusive_zone: 400,
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
            binded_output_name,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Counter {
    value: i32,
    text: String,
}

#[derive(Debug, Clone, Copy)]
enum WindowDirection {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    TextInput(String),
    Direction(WindowDirection),
    SizeChange((u32, u32)),
    IcedEvent(Event),
}

impl TryInto<LayershellCustomActions> for Message {
    type Error = Self;
    fn try_into(self) -> Result<LayershellCustomActions, Self::Error> {
        match self {
            Self::Direction(direction) => Ok(match direction {
                WindowDirection::Left => LayershellCustomActions::AnchorChange(
                    Anchor::Left | Anchor::Top | Anchor::Bottom,
                ),
                WindowDirection::Top => LayershellCustomActions::AnchorChange(
                    Anchor::Top | Anchor::Left | Anchor::Right,
                ),
                WindowDirection::Right => LayershellCustomActions::AnchorChange(
                    Anchor::Top | Anchor::Bottom | Anchor::Right,
                ),
                WindowDirection::Bottom => LayershellCustomActions::AnchorChange(
                    Anchor::Bottom | Anchor::Left | Anchor::Right,
                ),
            }),
            Self::SizeChange((x, y)) => Ok(LayershellCustomActions::SizeChange((x, y))),
            _ => Err(self),
        }
    }
}

impl Application for Counter {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
                text: "eee".to_string(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("Counter - Iced")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IcedEvent(event) => {
                println!("hello {event:?}");
                Command::none()
            }
            Message::IncrementPressed => {
                self.value += 1;
                Command::none()
            }
            Message::DecrementPressed => {
                self.value -= 1;
                Command::none()
            }
            Message::TextInput(text) => {
                self.text = text;
                Command::none()
            }

            Message::Direction(direction) => match direction {
                WindowDirection::Left => Command::batch(vec![
                    Command::done(message),
                    Command::done(Message::SizeChange((400, 0))),
                ]),
                WindowDirection::Right => Command::batch(vec![
                    Command::done(message),
                    Command::done(Message::SizeChange((400, 0))),
                ]),
                WindowDirection::Bottom => Command::batch(vec![
                    Command::done(message),
                    Command::done(Message::SizeChange((0, 400))),
                ]),
                WindowDirection::Top => Command::batch(vec![
                    Command::done(message),
                    Command::done(Message::SizeChange((0, 400))),
                ]),
            },
            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let center = column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20);
        row![
            button("left")
                .on_press(Message::Direction(WindowDirection::Left))
                .height(Length::Fill),
            column![
                button("top")
                    .on_press(Message::Direction(WindowDirection::Top))
                    .width(Length::Fill),
                center,
                text_input("hello", &self.text)
                    .on_input(Message::TextInput)
                    .padding(10),
                button("bottom")
                    .on_press(Message::Direction(WindowDirection::Bottom))
                    .width(Length::Fill),
            ]
            .width(Length::Fill),
            button("right")
                .on_press(Message::Direction(WindowDirection::Right))
                .height(Length::Fill),
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
}
