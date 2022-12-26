use std::sync::{Mutex, Arc};
use std::time::{Duration, Instant};

use iced::{executor, window, Font};
use iced::widget::{button, column, container, progress_bar, text, Column, pick_list, slider, row, text_input, Text, Container};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use iced_aw::{color_picker, TabLabel, Tabs};
use iced_aw::style::TabBarStyles;
use macropad_configurator::led_effects::LedRunner;
use macropad_configurator::macro_parser::LedConfig;
use macropad_configurator::{hid_manager, macro_parser, macropad};
use macropad_protocol::data_protocol::LedEffect;
use num_enum::{IntoPrimitive, FromPrimitive};

const ROBOTO: Font = iced::Font::External {
    name: "Roboto",
    bytes: include_bytes!("../../assets/fonts/Roboto-Regular.ttf"),
};

const ROBOTO_BOLD: Font = iced::Font::External {
    name: "Roboto Bold",
    bytes: include_bytes!("../../assets/fonts/Roboto-Bold.ttf"),
};

const ICON_FONT: Font = iced::Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/fonts/remixicon.ttf"),
};

#[derive(Debug, Clone)]
enum Icon {
    Keyboard,
    Led,
    Settings,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Keyboard => '\u{ee72}',
            Icon::Led => '\u{eea6}',
            Icon::Settings => '\u{f0e5}',
        }
    }
}

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

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
    key_tab: KeyTab,
    led_tab: LedTab,
    settings_tab: SettingsTab,
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
    ButtonPressed(usize),
    ReturnToMainPage,
    LedUpdate(Instant),
    LedEffectChanged(LedEffect),  
    LedPeriodChanged(f32),
    LedPeriodChangedText(String),
    TabSelected(usize),
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
                key_tab: KeyTab::default(),
                led_tab: LedTab::default(),
                settings_tab: SettingsTab::default(),
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
                self.key_tab = KeyTab::new(connection.get_macropad());
                self.led_tab = LedTab::new(connection.get_macropad(), LedRunner::default());
                self.settings_tab = SettingsTab::new(connection.get_macropad());
                self.state = State::Connected(connection, Page::MainPage(0));
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
            Message::ButtonPressed(i) => {
                self.state = State::Connected(
                    match &self.state {
                        State::Connected(connection, _) => connection.clone(),
                        _ => unreachable!(),
                    },
                    Page::ModifyKey(i),
                );
            }
            Message::ReturnToMainPage => {
                self.state = State::Connected(
                    match &mut self.state {
                        State::Connected(connection, _) => connection.clone(),
                        _ => unreachable!(),
                    },
                    Page::MainPage(0),
                );
            }
            Message::LedUpdate(_) => {
                if let State::Connected(con, Page::MainPage(id)) = &mut self.state {
                    if TabId::from(*id) == TabId::ModifyLed {
                        self.led_tab.led_runner.update(&con.get_macropad().lock().unwrap().led_config);
                    }
                }
            }
            Message::LedEffectChanged(effect) => {
                if let State::Connected(con, _) = &mut self.state {
                    con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::LedEffect(effect)));
                }
            } 
            Message::LedPeriodChanged(period) => {
                if let State::Connected(con, _) = &mut self.state {
                    con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::LedEffectPeriod(period / 10.0)));
                }
            }
            Message::LedPeriodChangedText(text) => {
                if let State::Connected(con, _) = &mut self.state {
                    if let Ok(period) = text.parse::<f32>() {
                        con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::LedEffectPeriod(period)));
                    }
                }
            }
            Message::TabSelected(i) => {
                if TabId::from(i) == TabId::ModifyLed {
                    self.led_tab.led_runner.reset();
                }
                self.state = State::Connected(
                    match &mut self.state {
                        State::Connected(connection, _) => {
                            connection.clone()
                        },
                        _ => unreachable!(),
                    },
                    Page::MainPage(i),
                );
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            hid_manager::connect().map(Message::HidEvent),
            match &self.state {
                State::Connected(_, Page::MainPage(i)) => {
                    if TabId::from(*i) == TabId::ModifyLed {
                        iced::time::every(Duration::from_millis(16))
                            .map(Message::LedUpdate)
                    } else {
                        Subscription::none()
                    }
                },
                _ => Subscription::none(),
            },
        ])
    }

    fn view(&self) -> Element<Message> {
        match &self.state {
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
            State::Connected(con, Page::MainPage(i)) => {
                Tabs::new(*i, Message::TabSelected)
                    .push(self.key_tab.tab_label(), self.key_tab.view())
                    .push(self.led_tab.tab_label(), self.led_tab.view())
                    .push(self.settings_tab.tab_label(), self.settings_tab.view())
                    .tab_bar_style(TabBarStyles::Purple)
                    .icon_font(ICON_FONT)
                    .tab_bar_position(iced_aw::TabBarPosition::Bottom)
                    .into()
            },
            // State::Connected(_, Page::MainPage) => {
            //     let message = column![
            //         text("Main Page")
            //             .font(ROBOTO)
            //             .size(60)
            //             .width(Length::Fill)
            //             .horizontal_alignment(iced::alignment::Horizontal::Center),
            //         text("This is the main page")
            //             .font(ROBOTO)
            //             .size(30)
            //             .width(Length::Fill)
            //             .horizontal_alignment(iced::alignment::Horizontal::Center),
                    
            //         button("Modify LEDs").on_press(Message::ButtonLedPage),
            //         // macropad::macropad_led([iced::Color::from_rgb(1.0, 0.0, 0.5); 4])
            //         macropad::macropad_button().on_press(Message::ButtonPressed)
            //             // .width(Length::Fill)
            //             // .height(Length::Fill)
            //             // .center_x()
            //             // .center_y()
            //             // .padding(20)
            //     ];

            //     container(message)
            //         // .width(Length::Shrink)
            //         // .height(Length::Shrink)
            //         .center_x()
            //         .center_y()
            //         .padding(100)
            //         .into()
            // },
            // State::Connected(con, Page::ModifyLeds(runner)) => {
            //     let config = con.get_macropad().lock().unwrap().led_config.clone();
            //     let tab = Tabs::new(self.active_tab, Message::TabSelected)
            //     .push(self.login_tab.tab_label(), self.login_tab.view())
            //     .push(self.ferris_tab.tab_label(), self.ferris_tab.view())
            //     .push(self.counter_tab.tab_label(), self.counter_tab.view())
            //     .push(self.settings_tab.tab_label(), self.settings_tab.view())
            //     .tab_bar_style(TabBarStyles::Default)
            //     .icon_font(ICON_FONT)
            //     .tab_bar_position(iced_aw::TabBarPosition::Bottom)
            //     .into();
                
            //     let message = column![
            //         text("Main Page")
            //             .font(ROBOTO)
            //             .size(60)
            //             .width(Length::Fill)
            //             .horizontal_alignment(iced::alignment::Horizontal::Center),
            //         text("This is the main page")
            //             .font(ROBOTO)
            //             .size(30)
            //             .width(Length::Fill)
            //             .horizontal_alignment(iced::alignment::Horizontal::Center),
            //         button("Main Page").on_press(Message::ButtonMainPage),

            //         column![
            //             text("Effect"),
            //             pick_list(&macropad_configurator::macropad_wrapper::EFFECTS[..], 
            //                 Some(config.effect), 
            //                 Message::LedEffectChanged),
            //         ],

                    
            //         column![
            //             text("Period"),
            //             row![
            //                 slider(-50.0..=50.0, config.effect_period * 10.0, Message::LedPeriodChanged),
            //                 text_input(config.effect_period.to_string().as_str(), config.effect_period.to_string().as_str(), Message::LedPeriodChangedText).width(Length::Units(100)),
            //             ],
            //         ],
                    
            //         column![
            //             text("Base Color"),
            //             row![
            //                 text("A")
            //             ],
            //         ],
            //         macropad::macropad_led(runner.get_leds(&config))
            //     ];

            //     container(message)
            //         // .width(Length::Shrink)
            //         // .height(Length::Shrink)
            //         .center_x()
            //         .center_y()
            //         .padding(100)
            //         .into()
            // },
            State::Connected(_, Page::ModifyKey(_)) => {
                let message = column![
                    text("Modify Key")
                        .font(ROBOTO)
                        .size(60)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("This is the modify key page")
                        .font(ROBOTO)
                        .size(30)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    button("Back")
                        .on_press(Message::ReturnToMainPage)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(20)
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(20)
                    .into()
            },
            State::Connected(_, Page::RecordMacro(_)) => todo!(),
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug)]
enum Page {
    MainPage(usize),
    // ModifyLeds(LedRunner),
    ModifyKey(usize),
    RecordMacro(usize),
}

#[repr(usize)]
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    IntoPrimitive,
    FromPrimitive,
)]
enum TabId {
    #[num_enum(default)]
    MainPage = 0,
    ModifyLed = 1,
    ModyifySettings = 2,
}

#[derive(Debug)]
enum State {
    Disconnected,
    Connected(hid_manager::Connection, Page),
}


trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}


#[derive(Debug)]
struct KeyTab {
    macropad: Option<Arc<Mutex<macro_parser::Macropad>>>,
}

impl KeyTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>) -> Self {
        Self { macropad: Some(macropad) }
    }
}

impl Default for KeyTab {
    fn default() -> Self {
        Self { macropad: None }
    }
}

impl Tab for KeyTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Keys")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Keyboard.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let macropad = {
            if let Some(macropad) = &self.macropad {
                macropad.lock().unwrap().clone()
            } else {
                panic!("Macropad not set");
            }
        };

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
            
            macropad::macropad_button().on_press(Message::ButtonPressed)
        ];

        container(message)
            // .width(Length::Shrink)
            // .height(Length::Shrink)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}

#[derive(Debug)]
struct LedTab {
    macropad: Option<Arc<Mutex<macro_parser::Macropad>>>,
    led_runner: LedRunner,
}

impl LedTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>, led_runner: LedRunner) -> Self {
        Self { macropad: Some(macropad), led_runner }
    }
}

impl Default for LedTab {
    fn default() -> Self {
        Self { macropad: None, led_runner: LedRunner::default() }
    }
}

impl Tab for LedTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("LEDs")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Led.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let config = {
            if let Some(macropad) = &self.macropad {
                macropad.lock().unwrap().led_config.clone()
            } else {
                panic!("Macropad not set");
            }
        };
        
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

            column![
                text("Effect"),
                pick_list(&macropad_configurator::macropad_wrapper::EFFECTS[..], 
                    Some(config.effect), 
                    Message::LedEffectChanged),
            ],

            
            column![
                text("Period"),
                row![
                    slider(-50.0..=50.0, config.effect_period * 10.0, Message::LedPeriodChanged),
                    text_input(config.effect_period.to_string().as_str(), config.effect_period.to_string().as_str(), Message::LedPeriodChangedText).width(Length::Units(100)),
                ],
            ],
            
            column![
                text("Base Color"),
                row![
                    text("A")
                ],
            ],
            macropad::macropad_led(self.led_runner.get_leds(&config))
        ];

        container(message)
            // .width(Length::Shrink)
            // .height(Length::Shrink)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}

#[derive(Debug)]
struct SettingsTab {
    macropad: Option<Arc<Mutex<macro_parser::Macropad>>>,
}

impl SettingsTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>) -> Self {
        Self { macropad: Some(macropad) }
    }
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self { macropad: None }
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Settings.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let macropad = {
            if let Some(macropad) = &self.macropad {
                macropad.lock().unwrap().clone()
            } else {
                panic!("Macropad not set");
            }
        };

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
            
            macropad::macropad_button().on_press(Message::ButtonPressed)
        ];

        container(message)
            // .width(Length::Shrink)
            // .height(Length::Shrink)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}