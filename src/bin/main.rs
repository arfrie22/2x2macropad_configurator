use std::sync::{Mutex, Arc};
use std::time::{Duration, Instant};

use iced::{executor, window, Font, Color};
use iced::widget::{button, column, container, progress_bar, text, Column, Row, pick_list, slider, row, text_input, Text, Container};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use iced_aw::{color_picker, TabLabel, Tabs, ColorPicker};
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
    Light,
    Gear,
    Sun,
    Moon,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Keyboard => '\u{ee72}',
            Icon::Light => '\u{eea6}',
            Icon::Gear => '\u{f0e5}',
            Icon::Sun => '\u{f1bc}',
            Icon::Moon => '\u{ef72}',
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
    LedBrightnessChanged(f32),
    LedBrightnessChangedText(String),
    PickColor,
    CancelColor,
    SubmitColor(Color),
    TabSelected(usize),
    PressTimeChangedText(String),
    HoldTimeChangedText(String),
    DefaultDelayChangedText(String),
    SwitchTheme,
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
                theme: match dark_light::detect() {
                    dark_light::Mode::Dark => Theme::Dark,
                    dark_light::Mode::Light => Theme::Light,
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
                self.settings_tab = SettingsTab::new(connection.get_macropad(), self.theme.clone());
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
            Message::LedBrightnessChanged(period) => {
                if let State::Connected(con, _) = &mut self.state {
                    con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::LedBrightness(period as u8)));
                }
            }
            Message::LedBrightnessChangedText(text) => {
                if let State::Connected(con, _) = &mut self.state {
                    if let Ok(brightness) = text.parse::<u8>() {
                        con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::LedBrightness(brightness)));
                    }
                }
            }
            Message::PickColor => {
                self.led_tab.show_picker = true;
            } 
            Message::CancelColor => {
                self.led_tab.show_picker = false;
            }
            Message::SubmitColor(color) => {
                if let State::Connected(con, _) = &mut self.state {
                    let c = color.into_rgba8();
                    con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::LedBaseColor((c[0], c[1], c[2]))));
                }
                self.led_tab.show_picker = false;
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
            Message::PressTimeChangedText(text) => {
                if let State::Connected(con, _) = &mut self.state {
                    if let Ok(press_time) = text.parse::<u32>() {
                        con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::TapSpeed(press_time * 1000)));
                    }
                }
            }
            Message::HoldTimeChangedText(text) => {
                if let State::Connected(con, _) = &mut self.state {
                    if let Ok(hold_time) = text.parse::<u32>() {
                        con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::HoldSpeed(hold_time * 1000)));
                    }
                }
            }
            Message::DefaultDelayChangedText(text) => {
                if let State::Connected(con, _) = &mut self.state {
                    if let Ok(default_delay) = text.parse::<u32>() {
                        con.send(hid_manager::Message::Set(hid_manager::MacropadCommand::DefaultDelay(default_delay * 1000)));
                    }
                }
            }
            Message::SwitchTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Dark,
                };
                self.settings_tab.theme = self.theme.clone();
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
                    .text_font(ROBOTO)
                    .text_size(20)
                    .into()
            },
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
            text("Select a key to modify")
                .font(ROBOTO)
                .size(60)
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            macropad::macropad_button().on_press(Message::ButtonPressed)
        ];

        container(message)
            .width(Length::Fill)
            .height(Length::Fill)
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
    show_picker: bool,
}

impl LedTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>, led_runner: LedRunner) -> Self {
        Self { macropad: Some(macropad), led_runner, show_picker: false }
    }
}

impl Default for LedTab {
    fn default() -> Self {
        Self { macropad: None, led_runner: LedRunner::default(), show_picker: false }
    }
}

impl Tab for LedTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("LEDs")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Light.into(), self.title())
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
            row![
                column![
                    container(column![
                        text("Effect").font(ROBOTO).size(30),
                        pick_list(&macropad_configurator::macropad_wrapper::EFFECTS[..], 
                            Some(config.effect), 
                            Message::LedEffectChanged),
                    ]).padding(20),
                    
                    container(column![
                        text("Period").font(ROBOTO).size(30),
                        row![
                            slider(-50.0..=50.0, config.effect_period * 10.0, Message::LedPeriodChanged).width(Length::Units(200)),
                            text_input(config.effect_period.to_string().as_str(), config.effect_period.to_string().as_str(), Message::LedPeriodChangedText).font(ROBOTO).width(Length::Units(50)),
                        ],
                    ]).padding(20),

                    container(column![
                        text("Brightness").font(ROBOTO).size(30),
                        row![
                            slider(0.0..=255.0, config.brightness as f32, Message::LedBrightnessChanged).width(Length::Units(200)),
                            text_input(config.brightness.to_string().as_str(), config.brightness.to_string().as_str(), Message::LedBrightnessChangedText).font(ROBOTO).width(Length::Units(50)),
                        ],
                    ]).padding(20),
                    
                    container(column![
                        text("Base Color").font(ROBOTO).size(30),
                        ColorPicker::new(
                            self.show_picker,
                            Color::from_rgb8(config.base_color.0, config.base_color.1, config.base_color.2),
                            button("Pick Color").on_press(Message::PickColor),
                            Message::CancelColor,
                            Message::SubmitColor,
                        )
                    ]).padding(20),
                ],

                macropad::macropad_led(self.led_runner.get_leds(&config)),
            ],
        ];

        container(message)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}

#[derive(Debug)]
struct SettingsTab {
    macropad: Option<Arc<Mutex<macro_parser::Macropad>>>,
    theme: Theme,
}

impl SettingsTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>, theme: Theme) -> Self {
        Self { 
            macropad: Some(macropad),
            theme
        }
    }
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self { 
            macropad: None,
            theme: match dark_light::detect() {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
            }
         }
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Gear.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let config = {
            if let Some(macropad) = &self.macropad {
                macropad.lock().unwrap().config.clone()
            } else {
                panic!("Macropad not set");
            }
        };

        // let a = 

        let message = column![
            container(column![
                text("Theme").font(ROBOTO).size(30),
                button(text(char::from(match self.theme {
                    Theme::Light => Icon::Sun,
                    Theme::Dark => Icon::Moon,
                    _ => Icon::Sun,
                })).font(ICON_FONT).size(30)).on_press(Message::SwitchTheme)
            ]).padding(20),

            container(column![
                text("Press Time (ms)").font(ROBOTO).size(30),
                text_input((config.tap_speed / 1000).to_string().as_str(), (config.tap_speed / 1000).to_string().as_str(), Message::PressTimeChangedText).font(ROBOTO).width(Length::Units(50)),
            ]).padding(20),

            container(column![
                text("Hold Time (ms)").font(ROBOTO).size(30),
                text_input((config.hold_speed / 1000).to_string().as_str(), (config.hold_speed / 1000).to_string().as_str(), Message::HoldTimeChangedText).font(ROBOTO).width(Length::Units(50)),
            ]).padding(20),

            container(column![
                text("Default Delay (ms)").font(ROBOTO).size(30),
                text_input((config.default_delay / 1000).to_string().as_str(), (config.default_delay / 1000).to_string().as_str(), Message::DefaultDelayChangedText).font(ROBOTO).width(Length::Units(50)),
            ]).padding(20),
        ];

        container(message)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}