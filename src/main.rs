use iced::{executor, window, Font};
use iced::widget::{button, column, container, progress_bar, text, Column};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};

// mod hidmanger;
mod macro_parser;
mod hid_manager;

const ROBOTO: Font = iced::Font::External {
    name: "Roboto",
    bytes: include_bytes!("../assets/fonts/Roboto-Regular.ttf"),
};

pub fn main() -> iced::Result {
    Configurator::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Configurator {
    state: State,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    HidMessage(hid_manager::Message),
    HidEvent(hid_manager::Event),
    MacroLoaded(macro_parser::Macro),
    MacroSaved,
    CommandSent(macropad_protocol::data_protocol::DataCommand, [u8; 64]),
    CommandReceived(macropad_protocol::data_protocol::DataCommand, [u8; 64]),
    CommandErrored,
}

impl Application for Configurator {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Configurator, Command<Message>) {
        (
            Configurator {
                state: State::Disconnected,
                theme: {
                    match dark_light::detect() {
                        dark_light::Mode::Dark => Theme::Dark,
                        dark_light::Mode::Light => Theme::Light,
                    }
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Macropad Configurator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::HidMessage(_) => {}
            Message::HidEvent(hid_manager::Event::Connected(connection)) => {
                self.state = State::Connected(connection, Page::MainPage);
            }
            Message::HidEvent(hid_manager::Event::Disconnected) => {
                self.state = State::Disconnected;
            }
            Message::HidEvent(_) => {}
            Message::MacroLoaded(_) => {}
            Message::MacroSaved => {}
            Message::CommandSent(_, _) => {}
            Message::CommandReceived(_, _) => {}
            Message::CommandErrored => {}
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        hid_manager::connect().map(Message::HidEvent)
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            State::Disconnected => {
                let message = column![
                    text("Disconnected")
                        .font(ROBOTO)
                        .size(60)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("Connect your macropad to get started")
                        .font(ROBOTO)
                        .size(30)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(20)
                    .into()
            },
            State::Connected(_, Page::MainPage) => {
                let message = column![
                    text("Main Page")
                        .font(ROBOTO)
                        .size(60)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("This is the main page")
                        .font(ROBOTO)
                        .size(30)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(20)
                    .into()
            },
            State::Connected(_, Page::ModifyKey(_)) => todo!(),
            State::Connected(_, Page::RecordMacro(_)) => todo!(),
            State::Connected(_, Page::ModifyLeds) => todo!(),
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug)]
enum Page {
    MainPage,
    ModifyKey(usize),
    RecordMacro(usize),
    ModifyLeds,
}

#[derive(Debug)]
enum State {
    Disconnected,
    Connected(hid_manager::Connection, Page),
}

// #[derive(Debug)]
// struct HidManager {
//     state: State,
// }

// #[derive(Debug)]
// enum State {
//     Idle,
//     WaitingForResponse,
//     Finished,
//     Errored,
// }

// impl HidManager {
//     pub fn new(id: usize) -> Self {
//         HidManager {
//             state: State::Idle,
//         }
//     }

//     pub fn start(&mut self) {
//         match self.state {
//             State::Idle { .. }
//             | State::WaitingForResponse { .. }
//             | State::Finished { .. }
//             | State::Errored { .. } => {
//                 self.state = State::Idle;
//             }
//             _ => {}
//         }
//     }

//     pub fn progress(&mut self, new_progress: download::Progress) {
//         if let State::Downloading { progress } = &mut self.state {
//             match new_progress {
//                 download::Progress::Started => {
//                     *progress = 0.0;
//                 }
//                 download::Progress::Advanced(percentage) => {
//                     *progress = percentage;
//                 }
//                 download::Progress::Finished => {
//                     self.state = State::Finished;
//                 }
//                 download::Progress::Errored => {
//                     self.state = State::Errored;
//                 }
//             }
//         }
//     }

//     pub fn subscription(&self) -> Subscription<Message> {
//         match self.state {
//             State::Downloading { .. } => {
//                 download::file(self.id, "https://speed.hetzner.de/100MB.bin?")
//                     .map(Message::DownloadProgressed)
//             }
//             _ => Subscription::none(),
//         }
//     }

//     pub fn view(&self) -> Element<Message> {
//         let current_progress = match &self.state {
//             State::Idle { .. } => 0.0,
//             State::Downloading { progress } => *progress,
//             State::Finished { .. } => 100.0,
//             State::Errored { .. } => 0.0,
//         };

//         let progress_bar = progress_bar(0.0..=100.0, current_progress);

//         let control: Element<_> = match &self.state {
//             State::Idle => button("Start the download!")
//                 .on_press(Message::Download(self.id))
//                 .into(),
//             State::Finished => {
//                 column!["Download finished!", button("Start again")]
//                     .spacing(10)
//                     .align_items(Alignment::Center)
//                     .into()
//             }
//             State::Downloading { .. } => {
//                 text(format!("Downloading... {:.2}%", current_progress)).into()
//             }
//             State::Errored => column![
//                 "Something went wrong :(",
//                 button("Try again").on_press(Message::Download(self.id)),
//             ]
//             .spacing(10)
//             .align_items(Alignment::Center)
//             .into(),
//         };

//         Column::new()
//             .spacing(10)
//             .padding(10)
//             .align_items(Alignment::Center)
//             .push(progress_bar)
//             .push(control)
//             .into()
//     }
// }