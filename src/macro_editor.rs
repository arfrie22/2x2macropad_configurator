use std::ops::Sub;
use std::time::Duration;

use iced::{mouse, Color, Vector, Size};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{
    self, Canvas, Cursor, Frame, Geometry, Path, Stroke,
};
use iced::{Element, Length, Point, Rectangle, Theme};

use crate::font::{ROBOTO, Icon, ICON_FONT};
use crate::macro_parser::{MacroFrame, Macro, self};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Index {
    Index {
        action_index: usize,
        index: usize,
    },
    Nested {
        action_index: usize,
        index: usize,
        parents: Vec<Index>
    },
}

impl Index {
    pub fn get_top_left(&self, scroll_offset: Vector) -> Point {
        match self {
            Index::Index { action_index, .. } => {
                let y = *action_index as f32 * (ACTION_SIZE.height + ACTION_PADDING);

                Point::new(0.0, y) - scroll_offset
            },
            Index::Nested { action_index, parents, .. } => {
                let x = parents.len() as f32 * LOOP_PADDING;
                let y = *action_index as f32 * (ACTION_SIZE.height + ACTION_PADDING);

                Point::new(x, y) - scroll_offset
            },
        }
    }

    pub fn get_offset(&self, scroll_offset: Vector, point: Point) -> Option<Vector> {
        let top_left = self.get_top_left(scroll_offset);
        let bounds = Rectangle::new(top_left, ACTION_SIZE);
        
        if bounds.contains(point) {
            Some(point - top_left)
        } else {
            None
        }
    }

    const CLOSE_BUTTON_PADDING: f32 = 1.0;
    pub const CLOSE_BUTTON_OFFSET: Vector = Vector::new(ACTION_SIZE.width - ACTION_SIZE.height - Self::CLOSE_BUTTON_PADDING, Self::CLOSE_BUTTON_PADDING);
    pub const CLOSE_BUTTON_SIZE: Size = Size::new(ACTION_SIZE.height - (2.0 * Self::CLOSE_BUTTON_PADDING), ACTION_SIZE.height - (2.0 * Self::CLOSE_BUTTON_PADDING));

    pub fn on_close_button(offset: Vector) -> bool {
        let close_button_bounds = Rectangle::new(
            Point::ORIGIN + Self::CLOSE_BUTTON_OFFSET,
            Self::CLOSE_BUTTON_SIZE
        );

        close_button_bounds.contains(Point::ORIGIN + offset)
    }

    // Just remove it from the list the iterate though and change all the indexes correctly
    // Have a coutner to keep track of the nested count in a stack as well as the index

    // pub fn remove_from_macro_actions(&self, macro_: &mut Macro, actions: &mut Vec<MacroAction>) {
    //     match self {
    //         Index::Index { action_index, index } => {
    //             macro_.frames.remove(*index);
    //             let outer_index = *action_index;
    //             for action in actions.iter_mut() {
    //                 match &mut action.index {
    //                     Index::Index { action_index, index } => {
    //                         if *action_index > outer_index {
    //                             *action_index -= 1;
    //                             *index -= 1;
    //                         }
    //                     },
    //                     Index::Nested { action_index, parent } => {
    //                         parent.
    //                         if *action_index > outer_index {
    //                             *action_index -= 1;
    //                         }
    //                     },
    //                 }

    //                 actions.remove(*action_index);
    //             }
    //         },
    //         Index::Nested { action_index, index, parent, .. } => {
    //             let parent = parent.clone();
    //             let parent = parent.remove_from_macro(macro_);
    //             let parent = parent.unwrap();

    //             macro_.actions[*action_index].frames[*index].nested.remove(parent);
    //         },
    //     }
    // }

    pub fn new_index(action_index: usize, index: usize) -> Self {
        Index::Index { action_index, index }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    MoveFrame(Index, Index),
    RemoveFrame(Index),
    AddFrame(MacroFrame),
    SelectFrame(Option<Index>),
    ReleaseGrab,
    DragStart,
    Scroll(Vector),
}

#[derive(Default, Debug)]
pub struct State {
    cache: canvas::Cache,
    scroll_offset: Vector,
    size_bounds: Rectangle,
}

const ACTION_SIZE: Size = Size::new(600.0, 50.0);
const ACTION_PADDING: f32 = 10.0;
const LOOP_PADDING: f32 = 50.0;


impl State {
    pub fn view<'a>(&'a self, actions: &'a [MacroAction]) -> Element<'a, Message> {
        Canvas::new(Editor {
            state: self,
            actions,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn scroll_to(&mut self, offset: Vector) {
        self.scroll_offset = offset;
    }

    pub fn scroll_to_top(&mut self, actions: &[MacroAction]) {
        self.scroll_offset = Vector::new(0.0, 0.0);
    }

    pub fn scroll_to_bottom(&mut self, actions: &[MacroAction], bounds: &Rectangle) {
        let max_y = (actions.len() as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - ACTION_PADDING;
        self.scroll_offset = Vector::new(0.0, max_y.sub(bounds.height).max(0.0));
    }
}

#[derive(Debug, Clone)]
struct Drag {
    action: MacroAction,
    drag_offset: Vector,
    start_position: Point,
    to: Point,
}

impl Drag {
    fn draw(&self, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            self.action.draw(
                &mut frame,
                theme,
                cursor_position - self.drag_offset,
                &None,
            );
        }

        frame.into_geometry()
    }
}

struct Editor<'a> {
    state: &'a State,
    actions: &'a [MacroAction],
}

impl<'a> Editor<'a> {
    fn get_max_x_offset(&self, bounds: &Rectangle) -> f32 {
        let mut max_x = 0.0;
        for action in self.actions.iter() {
            let x = ACTION_SIZE.width + (LOOP_PADDING * match &action.index {
                Index::Index { .. } => 0.0,
                Index::Nested { parents, .. } => parents.len() as f32,
            });
                
            if x > max_x {
                max_x = x;
            }
        }

        max_x.sub(bounds.width).max(0.0)
    }

    fn get_max_y_offset(&self, bounds: &Rectangle) -> f32 {
        let max_y = (self.actions.len() as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - ACTION_PADDING;
        max_y.sub(bounds.height).max(0.0)
    }
}

impl<'a> canvas::Program<Message> for Editor<'a> {
    type State = (Option<Drag>, Option<Index>);

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let cursor_position =
            if let Some(position) = cursor.position_in(&bounds) {
                position
            } else {
                return (event::Status::Ignored, None);
            };

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::WheelScrolled { delta } => {
                        match delta {
                            mouse::ScrollDelta::Lines { x, y } => {
                                let scroll_offset = self.state.scroll_offset + Vector::new(x * 20.0, -y * 20.0);
                                Some(Message::Scroll(scroll_offset))
                            },
                            mouse::ScrollDelta::Pixels { x, y } => {
                                let scroll_offset = self.state.scroll_offset + Vector::new(x, -y);
                                let scroll_offset = Vector::new(scroll_offset.x.max(0.0).min(self.get_max_x_offset(&bounds)), scroll_offset.y.max(0.0).min(self.get_max_y_offset(&bounds)));
                                Some(Message::Scroll(scroll_offset))
                            },
                        }
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match *state {
                            (None, _) => {
                                for action in self.actions {
                                    if let Some(offset) = action.index.get_offset(self.state.scroll_offset, cursor_position) {
                                        if Index::on_close_button(offset) {
                                            *state = (None, None);
                                            println!("Close button pressed");
                                            return (event::Status::Captured, Some(Message::RemoveFrame(action.index.clone())));
                                        }

                                        *state = (Some(Drag {
                                            action: action.clone(),
                                            drag_offset: offset,
                                            start_position: cursor_position,
                                            to: cursor_position,
                                        }), None);

                                        return (event::Status::Captured, None);
                                    }
                                }

                                if state.1.is_some() {
                                    *state = (None, None);
                                    return (event::Status::Captured, Some(Message::SelectFrame(None)));
                                }
                                None
                            },
                            _ => None,
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if let Some(cursor_position) = cursor.position_in(&bounds) {
                            match state.0.take() {
                                Some(Drag { action, drag_offset, start_position, to, .. }) => {
                                    let mut message = None;
                                    if start_position == to && cursor_position != to {
                                        message = Some(Message::DragStart);
                                    }
                                    
                                    let to = cursor_position;
                                    *state = (Some(Drag { action, drag_offset, start_position, to }), None);
                                    return (event::Status::Captured, message);
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        match state.0.take() {
                            Some(Drag { action, start_position, to, .. }) => {
                                println!("Drag from {:?} to {:?}", start_position, to);
                                if start_position == to {
                                    *state = (None, Some(action.index.clone()));
                                    return (event::Status::Captured, Some(Message::SelectFrame(Some(action.index))));
                                }
                                // for action in self.actions {
                                //     if let Some(offset) = action.index.get_offset(self.state.scroll_offset, cursor_position) {
                                //         if Index::on_close_button(offset) {
                                //             return (event::Status::Captured, Some(Message::RemoveFrame(action.index.clone())));
                                //         }

                                //         return (event::Status::Captured, Some(Message::MoveFrame(from, action.index.clone())));
                                //     }
                                // }

                                return (event::Status::Captured, Some(Message::ReleaseGrab));
                            }
                            None => None,
                        }
                    }
                    _ => None,
                };

                (event::Status::Captured, message)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let content =
            self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
                MacroAction::draw_all(self.actions, frame, theme, self.state.scroll_offset, &state.1, &state.0);

                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default().with_width(2.0),
                );

                frame.fill_text(canvas::Text {
                    content: "Click to add a new curve".to_string(),
                    position: Point::ORIGIN,
                    color: Color::from_rgb8(0xFF, 0x00, 0x00),
                    size: 30.0,
                    font: ROBOTO,
                    horizontal_alignment: iced::alignment::Horizontal::Left,
                    vertical_alignment: iced::alignment::Vertical::Top,
                });
            });


            if let Some(drag) = state.0.as_ref() {
                let drag_action = drag.draw(&theme, bounds, cursor);
                let placeholder = {
                    let mut frame = Frame::new(bounds.size());

                    let position = drag.action.index.get_top_left(self.state.scroll_offset);
                    
                    frame.stroke(
                        &Path::rectangle(position, ACTION_SIZE),
                        Stroke::default().with_color(Color::from_rgb8(0x00, 0xFF, 0xFF)).with_width(3.0),
                    );

                    frame.into_geometry()
                };
                vec![content, drag_action, placeholder]
            } else {
                vec![content]
            }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        match &state.0 {
            Some(_) => mouse::Interaction::Grabbing,
            None => {
                if let Some(cursor_position) = cursor.position_in(&bounds) {
                    for action in self.actions {
                        if let Some(offset) = action.index.get_offset(self.state.scroll_offset, cursor_position) {
                            if Index::on_close_button(offset) {
                                return mouse::Interaction::Pointer;
                            } else {
                                return mouse::Interaction::Grab;
                            }
                        }
                    }
        
                    mouse::Interaction::default()
                } else {
                    mouse::Interaction::default()
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacroAction {
    action: crate::macro_parser::ActionType,
    delay: Option<Duration>,
    index: Index,
}

impl MacroAction {
    fn draw_all(actions: &[MacroAction], frame: &mut Frame, theme: &Theme, scroll_offset: Vector, selected: &Option<Index>, drag: &Option<Drag>) {
        for action in actions {
            if let Some(drag) = drag {
                if drag.action.index == action.index {
                    continue;
                }
            }

            let position = action.index.get_top_left(scroll_offset);
            action.draw(frame, theme, position, selected);
        }
    }

    fn draw(&self, frame: &mut Frame, theme: &Theme, position: Point, selected: &Option<Index>) {
        
            frame.fill_rectangle(position, ACTION_SIZE, {
                if Some(self.index.clone()) == *selected {
                    Color::from_rgb8(0x00, 0x00, 0xFF)
                } else {
                    Color::from_rgb8(0x00, 0xFF, 0x00)
                }
            });
    
            // TODO: Will be buggy until iced supports vectorial text https://github.com/iced-rs/iced/pull/1610
            frame.fill_text(canvas::Text {
                content: Icon::Close.into(),
                position: position + Index::CLOSE_BUTTON_OFFSET,
                color: Color::from_rgb8(0xFF, 0x00, 0x00),
                size: Index::CLOSE_BUTTON_SIZE.width,
                font: ICON_FONT,
                horizontal_alignment: iced::alignment::Horizontal::Left,
                vertical_alignment: iced::alignment::Vertical::Top,
            });
    
            // match macro_frame.action {
            //     crate::macro_parser::ActionType::Loop(frames, _) => {
            //         *offset = *offset + Vector::new(LOOP_PADDING, 0.0);
            //         draw_all_frames(frames.as_slice(), frame, theme, offset);
            //         *offset = *offset - Vector::new(LOOP_PADDING, 0.0);
            //     },
    
            //     _ => {},
            // }
    }

    pub fn new(action: crate::macro_parser::ActionType, delay: Option<Duration>, index: Index) -> Self {
        Self {
            action,
            delay,
            index,
        }
    }

    // fn from_frame(frame: MacroFrame, parent: Option<usize>, ) -> Vec<MacroAction> {
    //     let mut actions = Vec::new();

    //     for (index, frame) in frames.iter().enumerate() {
    //         let action = MacroAction {
    //             position: frame.position,
    //             action: frame.action.clone(),
    //             index,
    //             parent,
    //         };

    //         actions.push(action);

    //         if let crate::macro_parser::ActionType::Loop(frames, _) = &frame.action {
    //             actions.append(&mut MacroAction::from_frames(frames.as_slice(), Some(index)));
    //         }
    //     }

    //     actions
    // }

    pub fn from_macro(macro_data: &crate::macro_parser::Macro) -> Vec<MacroAction> {
        let mut actions = Vec::new();

        for (index, frame) in macro_data.frames.iter().enumerate() {
            let index = Index::Index { action_index: 0, index };
            let action = MacroAction {
                action: frame.action.clone(),
                delay: frame.delay,
                index: index.clone(),
            };

            actions.push(action);

            if let crate::macro_parser::ActionType::Loop(frames, _) = &frame.action {
                actions.append(&mut MacroAction::from_frames(frames.as_slice(), vec![index]));
            }
        }

        let mut out_parents = Vec::new();
        let mut previous_index = Index::Index { action_index: 0, index: 0 };
        for (index, mut action) in actions.iter_mut().enumerate() {
            match &mut action.index {
                Index::Index { action_index, .. } => {
                    *action_index = index;

                    previous_index = action.index.clone();
                },
                Index::Nested { action_index, parents, .. } => {
                    *action_index = index;

                    if out_parents.len() != parents.len() {
                        out_parents.push(previous_index.clone());
                    }

                    if parents.len() != out_parents.len() {
                        panic!("parents and out_parents are not the same length");
                    }

                    for (parent_index, parent) in parents.iter_mut().enumerate() {
                        *parent = out_parents[parent_index].clone();
                    }

                    previous_index = action.index.clone();
                },
            }
        }

        actions
    }

    fn from_frames(frames: &[MacroFrame], parents: Vec<Index>) -> Vec<MacroAction> {
        let mut actions = Vec::new();

        for (index, frame) in frames.iter().enumerate() {
            let index = if parents.is_empty() {
                Index::Index { action_index: 0, index }    
            } else {
                Index::Nested { action_index: 0, index, parents: parents.clone() }
            };
            
            let action = MacroAction {
                action: frame.action.clone(),
                delay: frame.delay,
                index: index.clone(),
            };

            actions.push(action);

            if let crate::macro_parser::ActionType::Loop(frames, _) = &frame.action {
                actions.append(&mut MacroAction::from_frames(frames.as_slice(), {
                    let mut parents = parents.clone();
                    parents.push(index);
                    parents
                }));
            }
        }

        let mut out_parents = Vec::new();
        let mut previous_index = Index::Index { action_index: 0, index: 0 };
        for (index, mut action) in actions.iter_mut().enumerate() {
            match &mut action.index {
                Index::Index { action_index, .. } => {
                    *action_index = index;

                    previous_index = action.index.clone();
                },
                Index::Nested { action_index, parents, .. } => {
                    *action_index = index;

                    if out_parents.len() != parents.len() {
                        out_parents.push(previous_index.clone());
                    }

                    if parents.len() != out_parents.len() {
                        panic!("parents and out_parents are not the same length");
                    }

                    for (parent_index, parent) in parents.iter_mut().enumerate() {
                        *parent = out_parents[parent_index].clone();
                    }

                    previous_index = action.index.clone();
                },
            }
        }

        actions
    }
}
