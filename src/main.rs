use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use iced::subscription::{events, events_with};
use iced::theme::Button;
use iced::widget::{
    button, column, container, pane_grid, pick_list, progress_bar, row, slider, text, text_input,
    Column, Container, Row, Space, Text, radio,
};
use iced::{alignment, executor, window, Color, Font, Padding, Size};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use iced_aw::style::TabBarStyles;
use iced_aw::{color_picker, ColorPicker, TabLabel, Tabs};
use iced_native::widget::space;
use macropad_configurator::font::{ROBOTO, ICON_FONT, Icon};
use macropad_configurator::hid_manager::Connection;
use macropad_configurator::led_effects::LedRunner;
use macropad_configurator::macro_editor::Action;
use macropad_configurator::macro_parser::{LedConfig, ActionType, MacroFrame};
use macropad_configurator::type_wrapper::{KeyboardWrapper, ConsumerWrapper};
use macropad_configurator::{hid_manager, macro_parser, macropad, macropad_wrapper, type_wrapper, macro_editor};
use macropad_protocol::data_protocol::LedEffect;
use macropad_protocol::macro_protocol;
use num_enum::{FromPrimitive, IntoPrimitive};
use usbd_human_interface_device::page::{Keyboard, Consumer};



const ACTION_DELAY: u64 = 200;


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
    EditorMessage(macro_editor::Message),
    CommandSent(macropad_protocol::data_protocol::DataCommand, [u8; 64]),
    CommandReceived(macropad_protocol::data_protocol::DataCommand, [u8; 64]),
    CommandErrored,
    ButtonPressed(usize),
    ButtonHovered(Option<usize>),
    ButtonClicked(bool),
    ReturnToMainPage,
    LedUpdate(Instant),
    UpdateTick(Instant),
    TabSelected(usize),
    KeyModeChanged(macropad_protocol::data_protocol::KeyMode),
    LoadMacro(macro_parser::MacroType),
    KeyboardDataChanged(KeyboardWrapper),
    ConsumerDataChanged(ConsumerWrapper),
    KeyPickColor,
    KeyCancelColor,
    KeySubmitColor(Color),
    LedEffectChanged(LedEffect),
    LedPeriodChanged(f32),
    LedPeriodChangedText(String),
    LedBrightnessChanged(f32),
    LedBrightnessChangedText(String),
    LedPickColor,
    LedCancelColor,
    LedSubmitColor(Color),
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
            Message::EditorMessage(macro_editor::Message::MoveFrame(old_index, new_index)) => {
                old_index.move_in_macro(new_index, &mut self.key_tab.editor_actions);
                self.key_tab.editor.request_redraw();
            }
            Message::EditorMessage(macro_editor::Message::RemoveFrame(index)) => {
                index.remove_from_macro(&mut self.key_tab.editor_actions);

                // let frame = MacroFrame {
                //     action: ActionType::Empty,
                //     delay: Some(Duration::ZERO),
                // };
                // index.add_to_macro(frame, &mut self.key_tab.editor_actions);
                self.key_tab.editor.request_redraw();
            }
            Message::EditorMessage(macro_editor::Message::AddFrame(frame, index)) => {
                index.add_to_macro(frame, &mut self.key_tab.editor_actions);
                self.key_tab.editor.request_redraw();
            }
            Message::EditorMessage(macro_editor::Message::SelectFrame(index)) => {
                self.key_tab.editor.request_redraw();
            }
            Message::EditorMessage(macro_editor::Message::ReleaseGrab) => {
                self.key_tab.editor.request_redraw();
            }
            Message::EditorMessage(macro_editor::Message::DragStart) => {
                self.key_tab.editor.request_redraw();
            }
            Message::EditorMessage(macro_editor::Message::Scroll(offset)) => {
                self.key_tab.editor.scroll_to(offset);
                self.key_tab.editor.request_redraw();
            }
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

                self.key_tab.show_picker = false;
            }
            Message::ButtonHovered(state) => {
                self.key_tab.selected_key = state;
            }
            Message::ButtonClicked(state) => {
                self.key_tab.clicked = state;
            }
            Message::ReturnToMainPage => {
                self.state = State::Connected(
                    match &mut self.state {
                        State::Connected(connection, _) => connection.clone(),
                        _ => unreachable!(),
                    },
                    Page::MainPage(0),
                );
                self.key_tab.selected_key = None;
                self.key_tab.show_picker = false;
                self.key_tab.clicked = false;
            }
            Message::LedUpdate(_) => {
                if let State::Connected(con, Page::MainPage(id)) = &mut self.state {
                    if TabId::from(*id) == TabId::ModifyLed {
                        self.led_tab
                            .led_runner
                            .update(&con.get_macropad().lock().unwrap().led_config);
                    }
                }
            }
            Message::UpdateTick(_) => {
                if let State::Connected(con, _) = &mut self.state {
                    self.key_tab.run_actions(con);
                    self.led_tab.run_actions(con);
                    self.settings_tab.run_actions(con);
                }
            }
            Message::TabSelected(i) => {
                self.state = State::Connected(
                    match &mut self.state {
                        State::Connected(connection, _) => {
                            match TabId::from(i) {
                                TabId::MainPage => {}
                                TabId::ModifyLed => {
                                    self.led_tab.led_runner.reset();
                                    self.led_tab.update_config(connection.get_macropad());
                                }
                                TabId::ModyifySettings => {
                                    self.settings_tab.update_config(connection.get_macropad());
                                }
                            }
                            connection.clone()
                        }
                        _ => unreachable!(),
                    },
                    Page::MainPage(i),
                );
            }
            Message::KeyModeChanged(mode) => {
                if let State::Connected(con, Page::ModifyKey(i)) = &mut self.state {
                    self.key_tab
                        .queue_action(hid_manager::MacropadCommand::KeyMode(*i as u8, mode));
                }
            }
            Message::LoadMacro(macro_type) => {
                if let State::Connected(con, Page::ModifyKey(i)) = &mut self.state {
                    let macros = con.get_macropad().lock().unwrap().macros[*i as usize].clone();
                    self.key_tab.active_macro = match macro_type {
                        macro_parser::MacroType::Tap => {
                            macros.tap.clone()
                        },
                        macro_parser::MacroType::Hold => {
                            macros.hold.clone()
                        },
                        macro_parser::MacroType::DoubleTap => {
                            macros.double_tap.clone()
                        },
                        macro_parser::MacroType::TapHold => {
                            macros.tap_hold.clone()
                        },
                    };
                    self.key_tab.editor_actions = macro_editor::Action::from_macro(&self.key_tab.active_macro);
                    self.key_tab.editor.reset_scroll();
                    self.key_tab.editor.request_redraw();

                    self.state = State::Connected(
                        con.clone(),
                        Page::EditMacro(*i, macro_type.clone()),
                    );
                }
            }
            Message::KeyboardDataChanged(data) => {
                if let State::Connected(con, Page::ModifyKey(i)) = &mut self.state {
                    self.key_tab
                        .queue_action(hid_manager::MacropadCommand::KeyboardData(*i as u8, data.into()));
                }
            }
            Message::ConsumerDataChanged(data) => {
                if let State::Connected(con, Page::ModifyKey(i)) = &mut self.state {
                    self.key_tab
                        .queue_action(hid_manager::MacropadCommand::ConsumerData(*i as u8, data.into()));
                }
            }
            Message::KeyPickColor => {
                if let State::Connected(con, Page::ModifyKey(i)) = &mut self.state {
                    self.key_tab.show_picker = true;
                }
            }
            Message::KeyCancelColor => {
                self.key_tab.show_picker = false;
            }
            Message::KeySubmitColor(color) => {
                if let State::Connected(con, Page::ModifyKey(i)) = &mut self.state {
                    let c = color.into_rgba8();
                    self.key_tab
                        .queue_action(hid_manager::MacropadCommand::KeyColor(*i as u8, (c[0], c[1], c[2])));
                }
                self.key_tab.show_picker = false;
            }
            Message::LedEffectChanged(effect) => {
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedEffect(effect));
            }
            Message::LedPeriodChanged(period) => {
                self.led_tab.period_text = (period / 10.0).to_string();
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedEffectPeriod(period / 10.0));
            }
            Message::LedPeriodChangedText(text) => {
                if let Ok(period) = text.parse::<f32>() {
                    if (-5.0..=5.0).contains(&period) {
                        self.led_tab.period_text = text.clone();
                        self.led_tab
                            .queue_action(hid_manager::MacropadCommand::LedEffectPeriod(period));
                    }
                } else if text == "" {
                    self.led_tab.period_text = text;
                }
            }
            Message::LedBrightnessChanged(brightness) => {
                self.led_tab.brightness_text = brightness.to_string();
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedBrightness(
                        brightness as u8,
                    ));
            }
            Message::LedBrightnessChangedText(text) => {
                if let Ok(brightness) = text.parse::<u8>() {
                    if (0..=255).contains(&brightness) {
                        self.led_tab.brightness_text = text;
                        self.led_tab
                            .queue_action(hid_manager::MacropadCommand::LedBrightness(brightness));
                    }
                } else if text == "" {
                    self.led_tab.brightness_text = text;
                }
            }
            Message::LedPickColor => {
                self.led_tab.show_picker = true;
            }
            Message::LedCancelColor => {
                self.led_tab.show_picker = false;
            }
            Message::LedSubmitColor(color) => {
                let c = color.into_rgba8();
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedBaseColor((
                        c[0], c[1], c[2],
                    )));
                self.led_tab.show_picker = false;
            }
            Message::PressTimeChangedText(text) => {
                if let Ok(speed) = text.parse::<f32>() {
                    let speed = (speed * 1000.0).trunc() / 1000.0;
                    if speed != 0.0 && (0.0..=u32::MAX as f32).contains(&speed) {
                        if text.contains(".") && text.split('.').last().unwrap().len() > 3 {
                            self.settings_tab.press_time_text = format!("{:.3}", speed);
                        } else {
                            self.settings_tab.press_time_text = text;
                        }
                        self.settings_tab
                            .queue_action(hid_manager::MacropadCommand::TapSpeed(
                                (speed * 1000.0) as u32,
                            ));
                    }
                } else if text == "" {
                    self.settings_tab.press_time_text = text;
                }
            }
            Message::HoldTimeChangedText(text) => {
                if let Ok(speed) = text.parse::<f32>() {
                    let speed = (speed * 1000.0).trunc() / 1000.0;
                    if speed != 0.0 && (0.0..=u32::MAX as f32).contains(&speed) {
                        if text.contains(".") && text.split('.').last().unwrap().len() > 3 {
                            self.settings_tab.hold_time_text = format!("{:.3}", speed);
                        } else {
                            self.settings_tab.hold_time_text = text;
                        }
                        self.settings_tab
                            .queue_action(hid_manager::MacropadCommand::HoldSpeed(
                                (speed * 1000.0) as u32,
                            ));
                    }
                } else if text == "" {
                    self.settings_tab.hold_time_text = text;
                }
            }
            Message::DefaultDelayChangedText(text) => {
                if let Ok(speed) = text.parse::<f32>() {
                    let speed = (speed * 1000.0).trunc() / 1000.0;
                    if speed != 0.0 && (0.0..=u32::MAX as f32).contains(&speed) {
                        if text.contains(".") && text.split('.').last().unwrap().len() > 3 {
                            self.settings_tab.default_delay_text = format!("{:.3}", speed);
                        } else {
                            self.settings_tab.default_delay_text = text;
                        }
                        self.settings_tab
                            .queue_action(hid_manager::MacropadCommand::DefaultDelay(
                                (speed * 1000.0) as u32,
                            ));
                    }
                } else if text == "" {
                    self.settings_tab.default_delay_text = text;
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
                    iced::time::every(Duration::from_millis(16)).map(Message::LedUpdate)
                }
                _ => Subscription::none(),
            },
            match &self.state {
                State::Connected(con, _) => {
                    iced::time::every(Duration::from_millis(50)).map(Message::UpdateTick)
                }
                _ => Subscription::none(),
            },
        ])
    }

    fn view(&self) -> Element<Message> {
        match &self.state {
            State::Disconnected => {
                // TODO: Add ability to flash firmware
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
            }
            State::Connected(_, Page::MainPage(i)) => Tabs::new(*i, Message::TabSelected)
                .push(self.key_tab.tab_label(), self.key_tab.view())
                .push(self.led_tab.tab_label(), self.led_tab.view())
                .push(self.settings_tab.tab_label(), self.settings_tab.view())
                .tab_bar_style(TabBarStyles::Purple)
                .icon_font(ICON_FONT)
                .tab_bar_position(iced_aw::TabBarPosition::Bottom)
                .text_font(ROBOTO)
                .text_size(20)
                .into(),
            State::Connected(_, Page::ModifyKey(i)) => {
                let key_settings = match self.key_tab.key_configs[*i].key_mode {
                    macropad_protocol::data_protocol::KeyMode::MacroMode => {
                        column![
                            container(column![
                                text("Key Mode").font(ROBOTO).size(30),
                                row![
                                    button("Tap Macro").on_press(Message::LoadMacro(macro_parser::MacroType::Tap)),
                                    Space::with_width(Length::Units(20)),
                                    button("Hold Macro").on_press(Message::LoadMacro(macro_parser::MacroType::Hold)),
                                    Space::with_width(Length::Units(20)),
                                    button("Double Tap Macro").on_press(Message::LoadMacro(macro_parser::MacroType::DoubleTap)),
                                    Space::with_width(Length::Units(20)),
                                    button("Tap and Hold Macro").on_press(Message::LoadMacro(macro_parser::MacroType::TapHold)),
                                ],
                            ])
                            .padding(Padding {
                                top: 20,
                                right: 0,
                                bottom: 20,
                                left: 0,
                            }),
                        ]
                    },
                    macropad_protocol::data_protocol::KeyMode::SingleTapMode => {
                        column![
                            container(column![
                                text("Key Mode").font(ROBOTO).size(30),
                                row![
                                    button("Tap Macro").on_press(Message::LoadMacro(macro_parser::MacroType::Tap)),
                                    Space::with_width(Length::Units(20)),
                                    button("Hold Macro").on_press(Message::LoadMacro(macro_parser::MacroType::Hold)),
                                    Space::with_width(Length::Units(20)),
                                    button("Double Tap Macro"),
                                    Space::with_width(Length::Units(20)),
                                    button("Tap and Hold Macro"),
                                ],
                            ])
                            .padding(Padding {
                                top: 20,
                                right: 0,
                                bottom: 20,
                                left: 0,
                            }),
                        ]
                    },
                    macropad_protocol::data_protocol::KeyMode::KeyboardMode => {
                        column![
                            container(column![
                                text("Key").font(ROBOTO).size(30),
                                pick_list(
                                    &type_wrapper::KeyboardWrapper::KEYS[..],
                                    Some(self.key_tab.key_configs[*i].keyboard_data.into()),
                                    Message::KeyboardDataChanged
                                ),
                            ])
                            .padding(Padding {
                                top: 20,
                                right: 0,
                                bottom: 20,
                                left: 0,
                            }),
                            container(column![
                                text("Key Color").font(ROBOTO).size(30),
                                ColorPicker::new(
                                    self.key_tab.show_picker,
                                    Color::from_rgb8(
                                        self.key_tab.key_configs[*i].key_color.0,
                                        self.key_tab.key_configs[*i].key_color.1,
                                        self.key_tab.key_configs[*i].key_color.2
                                    ),
                                    button("Pick Color").on_press(Message::KeyPickColor),
                                    Message::KeyCancelColor,
                                    Message::KeySubmitColor,
                                )
                            ])
                            .padding(Padding {
                                top: 20,
                                right: 0,
                                bottom: 20,
                                left: 0,
                            }),
                        ]
                    },
                    macropad_protocol::data_protocol::KeyMode::ConsumerMode => {
                        column![
                            container(column![
                                text("Consumer").font(ROBOTO).size(30),
                                pick_list(
                                    &type_wrapper::ConsumerWrapper::KEYS[..],
                                    Some(self.key_tab.key_configs[*i].consumer_data.into()),
                                    Message::ConsumerDataChanged
                                ),
                            ])
                            .padding(Padding {
                                top: 20,
                                right: 0,
                                bottom: 20,
                                left: 0,
                            }),
                            container(column![
                                text("Key Color").font(ROBOTO).size(30),
                                ColorPicker::new(
                                    self.key_tab.show_picker,
                                    Color::from_rgb8(
                                        self.key_tab.key_configs[*i].key_color.0,
                                        self.key_tab.key_configs[*i].key_color.1,
                                        self.key_tab.key_configs[*i].key_color.2
                                    ),
                                    button("Pick Color").on_press(Message::KeyPickColor),
                                    Message::KeyCancelColor,
                                    Message::KeySubmitColor,
                                )
                            ])
                            .padding(Padding {
                                top: 20,
                                right: 0,
                                bottom: 20,
                                left: 0,
                            }),
                        ]
                    },
                };

                
                let selected_key_mode = Some(self.key_tab.key_configs[*i].key_mode);
                let message = column![
                    container(
                        row![
                            container(button("Back")
                                .on_press(Message::ReturnToMainPage)
                                .width(Length::Shrink)
                                .height(Length::Shrink)
                            ).align_x(iced::alignment::Horizontal::Left),
                            text(format!("Modify Key {}", i))
                                .font(ROBOTO)
                                .size(60)
                                .width(Length::Fill)
                                .horizontal_alignment(iced::alignment::Horizontal::Center),
                        ]).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Top),
                    
                    container(column![
                        container(column![
                            text("Key Mode").font(ROBOTO).size(30),
                            row![
                                radio("Macro Mode", macropad_protocol::data_protocol::KeyMode::MacroMode, selected_key_mode, Message::KeyModeChanged),
                                Space::with_width(Length::Units(20)),
                                radio("Single Tap Mode", macropad_protocol::data_protocol::KeyMode::SingleTapMode, selected_key_mode, Message::KeyModeChanged),
                                Space::with_width(Length::Units(20)),
                                radio("Keyboard Mode", macropad_protocol::data_protocol::KeyMode::KeyboardMode, selected_key_mode, Message::KeyModeChanged),
                                Space::with_width(Length::Units(20)),
                                radio("Consumer Mode", macropad_protocol::data_protocol::KeyMode::ConsumerMode, selected_key_mode, Message::KeyModeChanged),
                            ],
                        ])
                        .padding(Padding {
                            top: 20,
                            right: 0,
                            bottom: 20,
                            left: 0,
                        }),
                        key_settings,
                    ]).width(Length::Fill).height(Length::Fill).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Top).padding(20)
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(100)
                    .into()
            }
            State::Connected(_, Page::EditMacro(i, macro_type)) => {
                let message = column![
                    text(format!("Edit {:?} Macro {:?}", macro_type, i)).font(ROBOTO).size(60),
                    button("Back").on_press(Message::ButtonPressed(*i)),
                    self.key_tab.editor.view(&self.key_tab.editor_actions.as_slice()).map(Message::EditorMessage),
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(100)
                    .into()
                // todo!()
            }
        }
        
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug)]
enum Page {
    MainPage(usize),
    ModifyKey(usize),
    EditMacro(usize, macro_parser::MacroType),
}

#[repr(usize)]
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoPrimitive, FromPrimitive,
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
    selected_key: Option<usize>,
    clicked: bool,
    show_picker: bool,
    key_configs: Vec<macro_parser::KeyConfig>,
    macros: Vec<macro_parser::MacroCollection>,
    active_macro: macro_parser::Macro,
    editor: macro_editor::State,
    editor_actions: Vec<Action>,
    actions: HashMap<
        macropad_protocol::data_protocol::KeyConfigElements,
        (bool, Instant, hid_manager::MacropadCommand),
    >,
}

impl KeyTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>) -> Self {
        let macropad = macropad.lock().unwrap().clone();
        Self {
            selected_key: None,
            clicked: false,
            show_picker: false,
            key_configs: macropad.key_configs.clone(),
            macros: macropad.macros.clone(),
            active_macro: macro_parser::Macro::default(),
            editor: macro_editor::State::default(),
            editor_actions: Vec::new(),
            actions: HashMap::new(),
        }
    }

    fn update_config(&mut self, macropad: Arc<Mutex<macro_parser::Macropad>>) {
        let macropad = macropad.lock().unwrap().clone();
        self.key_configs = macropad.key_configs.clone();
        self.macros = macropad.macros.clone();
    }

    fn run_actions(&mut self, con: &mut Connection) {
        for (command, (active, time_to_run, action)) in self.actions.iter_mut() {
            if *active && time_to_run.elapsed() > Duration::ZERO {
                *active = false;

                con.send(hid_manager::Message::Set(action.clone()));
            }
        }
    }

    fn queue_action(&mut self, action: hid_manager::MacropadCommand) {
        match action {
            hid_manager::MacropadCommand::KeyMode(key, mode) => {
                self.key_configs[key as usize].key_mode = mode;
                self.actions.insert(
                    macropad_protocol::data_protocol::KeyConfigElements::KeyMode,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::KeyboardData(key, keyboard) => {
                self.key_configs[key as usize].keyboard_data = keyboard;
                self.actions.insert(
                    macropad_protocol::data_protocol::KeyConfigElements::KeyboardData,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::ConsumerData(key, consumer) => {
                self.key_configs[key as usize].consumer_data = consumer;
                self.actions.insert(
                    macropad_protocol::data_protocol::KeyConfigElements::ConsumerData,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::KeyColor(key, color) => {
                self.key_configs[key as usize].key_color = color;
                self.actions.insert(
                    macropad_protocol::data_protocol::KeyConfigElements::KeyColor,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            _ => unreachable!(),
        }
    }
}

impl Default for KeyTab {
    fn default() -> Self {
        Self {
            selected_key: None,
            clicked: false,
            show_picker: false,
            key_configs: Vec::new(),
            macros: Vec::new(),
            active_macro: macro_parser::Macro::default(),
            editor: macro_editor::State::default(),
            editor_actions: Vec::new(),
            actions: HashMap::new(),
        }
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
        let message = column![
            text("Select a key to modify")
                .font(ROBOTO)
                .size(60)
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            macropad::macropad_button(self.selected_key, self.clicked).on_press(Message::ButtonPressed).on_hover(Message::ButtonHovered).on_click(Message::ButtonClicked),
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
    config: macro_parser::LedConfig,
    led_runner: LedRunner,
    show_picker: bool,
    period_text: String,
    brightness_text: String,
    actions: HashMap<
        macropad_protocol::data_protocol::LedCommand,
        (bool, Instant, hid_manager::MacropadCommand),
    >,
}

impl LedTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>, led_runner: LedRunner) -> Self {
        let config = macropad.lock().unwrap().led_config.clone();

        Self {
            config: config.clone(),
            led_runner,
            show_picker: false,
            period_text: config.effect_period.to_string(),
            brightness_text: config.brightness.to_string(),
            actions: HashMap::new(),
        }
    }

    fn update_config(&mut self, macropad: Arc<Mutex<macro_parser::Macropad>>) {
        self.config = macropad.lock().unwrap().led_config.clone();
    }

    fn run_actions(&mut self, con: &mut Connection) {
        for (command, (active, time_to_run, action)) in self.actions.iter_mut() {
            if *active && time_to_run.elapsed() > Duration::ZERO {
                *active = false;

                con.send(hid_manager::Message::Set(action.clone()));
            }
        }
    }

    fn queue_action(&mut self, action: hid_manager::MacropadCommand) {
        match action {
            hid_manager::MacropadCommand::LedBaseColor(color) => {
                self.config.base_color = color;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::BaseColor,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedEffect(effect) => {
                self.config.effect = effect;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::Effect,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedBrightness(brightness) => {
                self.config.brightness = brightness;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::Brightness,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedEffectPeriod(period) => {
                self.config.effect_period = period;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::EffectPeriod,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedEffectOffset(offset) => {
                self.config.effect_offset = offset;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::EffectOffset,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            _ => unreachable!(),
        }
    }
}

impl Default for LedTab {
    fn default() -> Self {
        Self {
            config: LedConfig::default(),
            led_runner: LedRunner::default(),
            show_picker: false,
            period_text: String::from(""),
            brightness_text: String::from(""),
            actions: HashMap::new(),
        }
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
        let message = column![row![
            column![
                container(column![
                    text("Effect").font(ROBOTO).size(30),
                    pick_list(
                        &macropad_configurator::macropad_wrapper::EFFECTS[..],
                        Some(self.config.effect),
                        Message::LedEffectChanged
                    ),
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Period").font(ROBOTO).size(30),
                    row![
                        slider(
                            -50.0..=50.0,
                            self.config.effect_period * 10.0,
                            Message::LedPeriodChanged
                        )
                        .width(Length::Units(200)),
                        Space::with_width(Length::Units(20)),
                        text_input(
                            self.config.effect_period.to_string().as_str(),
                            self.period_text.as_str(),
                            Message::LedPeriodChangedText
                        )
                        .font(ROBOTO)
                        .width(Length::Units(50)),
                    ],
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Brightness").font(ROBOTO).size(30),
                    row![
                        slider(
                            0.0..=255.0,
                            self.config.brightness as f32,
                            Message::LedBrightnessChanged
                        )
                        .width(Length::Units(200)),
                        Space::with_width(Length::Units(20)),
                        text_input(
                            self.config.brightness.to_string().as_str(),
                            self.brightness_text.as_str(),
                            Message::LedBrightnessChangedText
                        )
                        .font(ROBOTO)
                        .width(Length::Units(50)),
                    ],
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Base Color").font(ROBOTO).size(30),
                    ColorPicker::new(
                        self.show_picker,
                        Color::from_rgb8(
                            self.config.base_color.0,
                            self.config.base_color.1,
                            self.config.base_color.2
                        ),
                        button("Pick Color").on_press(Message::LedPickColor),
                        Message::LedCancelColor,
                        Message::LedSubmitColor,
                    )
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
            ],
            macropad::macropad_led(self.led_runner.get_leds(&self.config)),
        ],];

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
    config: macro_parser::MacroConfig,
    theme: Theme,
    press_time_text: String,
    hold_time_text: String,
    default_delay_text: String,
    actions: HashMap<
        macropad_protocol::data_protocol::ConfigElements,
        (bool, Instant, hid_manager::MacropadCommand),
    >,
}

impl SettingsTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>, theme: Theme) -> Self {
        let config = macropad.lock().unwrap().config.clone();

        Self {
            config: config.clone(),
            theme,
            press_time_text: (config.tap_speed / 1000).to_string(),
            hold_time_text: (config.hold_speed / 1000).to_string(),
            default_delay_text: (config.default_delay / 1000).to_string(),
            actions: HashMap::new(),
        }
    }

    fn update_config(&mut self, macropad: Arc<Mutex<macro_parser::Macropad>>) {
        self.config = macropad.lock().unwrap().config.clone();
    }

    fn run_actions(&mut self, con: &mut Connection) {
        for (command, (active, time_to_run, action)) in self.actions.iter_mut() {
            if *active && time_to_run.elapsed() > Duration::ZERO {
                *active = false;

                con.send(hid_manager::Message::Set(action.clone()));
            }
        }
    }

    fn queue_action(&mut self, action: hid_manager::MacropadCommand) {
        match action {
            hid_manager::MacropadCommand::TapSpeed(speed) => {
                self.config.tap_speed = speed;
                self.actions.insert(
                    macropad_protocol::data_protocol::ConfigElements::TapSpeed,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::HoldSpeed(speed) => {
                self.config.hold_speed = speed;
                self.actions.insert(
                    macropad_protocol::data_protocol::ConfigElements::HoldSpeed,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::DefaultDelay(delay) => {
                self.config.default_delay = delay;
                self.actions.insert(
                    macropad_protocol::data_protocol::ConfigElements::DefaultDelay,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            _ => unreachable!(),
        }
    }
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self {
            config: macro_parser::MacroConfig::default(),
            theme: match dark_light::detect() {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
            },
            press_time_text: String::from(""),
            hold_time_text: String::from(""),
            default_delay_text: String::from(""),
            actions: HashMap::new(),
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
        
        let message = column![
            container(button(
                text(char::from(match self.theme {
                    Theme::Light => Icon::Sun,
                    Theme::Dark => Icon::Moon,
                    _ => Icon::Sun,
                }))
                .font(ICON_FONT)
                .size(30)
                
            )
            .on_press(Message::SwitchTheme)
            .style(Button::Text)
            ).width(Length::Fill).align_x(alignment::Horizontal::Right).align_y(alignment::Vertical::Top),
            container(column![
                container(column![
                    text("Press Time (ms)").font(ROBOTO).size(30),
                    text_input(
                        (self.config.tap_speed / 1000).to_string().as_str(),
                        self.press_time_text.as_str(),
                        Message::PressTimeChangedText
                    )
                    .font(ROBOTO)
                    .width(Length::Units(50)),
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Hold Time (ms)").font(ROBOTO).size(30),
                    text_input(
                        (self.config.hold_speed / 1000).to_string().as_str(),
                        self.hold_time_text.as_str(),
                        Message::HoldTimeChangedText
                    )
                    .font(ROBOTO)
                    .width(Length::Units(50)),
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Default Delay (ms)").font(ROBOTO).size(30),
                    text_input(
                        (self.config.default_delay / 1000).to_string().as_str(),
                        self.default_delay_text.as_str(),
                        Message::DefaultDelayChangedText
                    )
                    .font(ROBOTO)
                    .width(Length::Units(50)),
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
            ]).width(Length::Fill).height(Length::Fill).align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Top).padding(20),
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

