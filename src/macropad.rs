use iced::{Background, Color};
// For now, to implement a custom native widget you will need to add
// `iced_native` and `iced_wgpu` to your dependencies.
//
// Then, you simply need to define your widget type and implement the
// `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
//
// Of course, you can choose to make the implementation renderer-agnostic,
// if you wish to, by creating your own `Renderer` trait, which could be
// implemented by `iced_wgpu` and other renderers.
use iced_native::layout::{self, Layout};
use iced_graphics::renderer::{self, Renderer};
use iced_graphics::triangle::ColoredVertex2D;
use iced_graphics::{Backend, Primitive};
use iced_native::renderer::BorderRadius;
use iced_native::widget::{self, Widget};
use iced_native::{Element, Length, Point, Rectangle, Size, Vector};


// For now, to implement a custom native widget you will need to add
// `iced_native` and `iced_wgpu` to your dependencies.
//
// Then, you simply need to define your widget type and implement the
// `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
//
// Of course, you can choose to make the implementation renderer-agnostic,
// if you wish to, by creating your own `Renderer` trait, which could be
// implemented by `iced_wgpu` and other renderers.


#[derive(Default)]
pub struct Macropad<'a, Message> {
    pub interactable: bool,
    pub glow: [Color; 4],   
    selected: Option<usize>,
    clicked: bool,
    message: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

pub fn macropad<'a, Message>(interactable: bool, glow: [Color; 4], message: Option<Box<dyn Fn(usize) -> Message + 'a>>) -> Macropad<'a, Message> {
    Macropad {
        interactable,
        glow,
        selected: None,
        clicked: false,
        message
    }
}

pub fn macropad_button<'a, Message>() -> Macropad<'a, Message> {
    macropad(true, [Color::TRANSPARENT; 4], None)
}

pub fn macropad_led<'a, Message>(glow: [Color; 4]) -> Macropad<'a, Message> {
    macropad(false, glow, None)
}

impl<'a, Message> Macropad<'a, Message> {

    pub fn on_press<F>(mut self, message: F) -> Self 
    where F: 'a + Fn(usize) -> Message,
    {
        self.message = Some(Box::new(message));
        self
    }

    fn get_keys(&self, b: &Rectangle) -> Vec<Rectangle<f32>> {
        let len = b.width.min(b.height);
        let offset = 30.0 / 70.0 * len;

        vec![
            Rectangle {
                x: b.center_x() - offset + ((35.0 - 16.525) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 - 16.525) / 70.0 * len),
                width: 14.0 / 70.0 * len,
                height: 14.0 / 70.0 * len,
            },
            Rectangle {
                x: b.center_x() - offset + ((35.0 - 16.525) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 + 2.525) / 70.0 * len),
                width: 14.0 / 70.0 * len,
                height: 14.0 / 70.0 * len,
            },
            Rectangle {
                x: b.center_x() - offset + ((35.0 + 2.525) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 + 2.525) / 70.0 * len),
                width: 14.0 / 70.0 * len,
                height: 14.0 / 70.0 * len,
            },
            Rectangle {
                x: b.center_x() - offset + ((35.0 + 2.525) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 - 16.525) / 70.0 * len),
                width: 14.0 / 70.0 * len,
                height: 14.0 / 70.0 * len,
            }
        ]
    }
}

impl<'a, Message, B, T> Widget<Message, Renderer<B, T>> for Macropad<'a, Message>
where
    B: Backend,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    fn layout(&self, _renderer: &Renderer<B, T>, limits: &layout::Limits) -> layout::Node {
        let size_w = limits.width(Length::Fill).resolve(Size::ZERO).width;
        let size_h = limits.height(Length::Fill).resolve(Size::ZERO).height;

        layout::Node::new(Size::new(size_w, size_h))
    }

    fn on_event(
            &mut self,
            _state: &mut widget::Tree,
            event: iced::Event,
            layout: Layout<'_>,
            cursor_position: Point,
            _renderer: &Renderer<B, T>,
            _clipboard: &mut dyn iced_native::Clipboard,
            shell: &mut iced_native::Shell<'_, Message>,
        ) -> iced::event::Status {
        if self.interactable {
            if !self.clicked {
                if let iced::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) = event {
                    for (i, key) in self.get_keys(&layout.bounds()).iter().enumerate() {
                        if key.contains(cursor_position) {
                            self.selected = Some(i);
                            return iced::event::Status::Captured;
                        }
                    }
                    self.selected = None;
                    self.clicked = false;
                }
            }

            if let Some(i) = self.selected {
                match event {
                    iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                        self.clicked = true;
                        return iced::event::Status::Captured;
                    }
                    iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                        if self.clicked {
                            if self.get_keys(&layout.bounds())[i].contains(cursor_position) {
                                shell.publish(self.message.as_ref().unwrap()(i));
                            }
                            
                            self.clicked = false;
                            return iced::event::Status::Captured;
                        }
                    }
                    _ => {}
                }
            }
        }

        iced::event::Status::Ignored
    }

    fn mouse_interaction(
            &self,
            _state: &widget::Tree,
            layout: Layout<'_>,
            cursor_position: Point,
            _viewport: &Rectangle,
            _renderer: &Renderer<B, T>,
        ) -> iced_native::mouse::Interaction {
            if self.interactable {
                for (i, key) in self.get_keys(&layout.bounds()).iter().enumerate() {
                    if key.contains(cursor_position) {
                        if let Some(selected) = self.selected {
                            if selected == i {
                                return iced::mouse::Interaction::Pointer;
                            } else {
                                break;
                            }
                        } else {
                            return iced::mouse::Interaction::Pointer;
                        }
                    }
                }
            }
            
            iced::mouse::Interaction::default()
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer<B, T>,
        _theme: &T,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        use iced_graphics::triangle::Mesh2D;
        use iced_native::Renderer as _;

        let b = layout.bounds();

        let len = b.width.min(b.height);
        let offset = 30.0 / 70.0 * len;

        let board = Primitive::Quad {
            bounds: Rectangle {
                x: b.center_x() - offset + 5.0 / 70.0 * len,
                y: b.center_y() - offset + 5.0 / 70.0 * len,
                width: 60.0 / 70.0 * len,
                height: 60.0 / 70.0 * len,
            },
            background: Background::Color(Color::from_rgb8(0x8D, 0x36, 0xCB)),
            border_radius: [5.0 / 70.0 * len; 4],
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };

        let plug = Primitive::Quad {
            bounds: Rectangle {
                x: b.center_x() - offset + 16.9 / 70.0 * len,
                y: b.center_y() - offset + 2.9 / 70.0 * len,
                width: 11.9 / 70.0 * len,
                height: 2.1 / 70.0 * len,
            },
            background: Background::Color(Color::from_rgb8(0x7B, 0x7B, 0x7B)),
            border_radius: [
                1.0 / 70.0 * len,
                1.0 / 70.0 * len,
                0.0,
                0.0,
                ],
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };

        const GLOW_D: f32 = 1.0;

        let glows = vec![
            Rectangle {
                x: b.center_x() - offset + ((35.0 - 16.525 - GLOW_D) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 - 16.525 - GLOW_D) / 70.0 * len),
                width: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
                height: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
            },
            Rectangle {
                x: b.center_x() - offset + ((35.0 - 16.525 - GLOW_D) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 + 2.525 - GLOW_D) / 70.0 * len),
                width: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
                height: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
            },
            Rectangle {
                x: b.center_x() - offset + ((35.0 + 2.525 - GLOW_D) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 + 2.525 - GLOW_D) / 70.0 * len),
                width: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
                height: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
            },
            Rectangle {
                x: b.center_x() - offset + ((35.0 + 2.525 - GLOW_D) / 70.0 * len),
                y: b.center_y() - offset + ((35.0 - 16.525 - GLOW_D) / 70.0 * len),
                width: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
                height: (14.0 + (GLOW_D * 2.0)) / 70.0 * len,
            }
        ];

        

        renderer.draw_primitive(plug);
        renderer.draw_primitive(board);

        for (i, glow) in glows.iter().enumerate() {
            renderer.draw_primitive(Primitive::Quad { 
                bounds: *glow,
                background: Background::Color(self.glow[i]),
                border_radius: [(0.5 + GLOW_D) / 70.0 * len; 4],
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
             });
        }

        for (i, key) in self.get_keys(&b).iter().enumerate() {
            renderer.draw_primitive(Primitive::Quad { 
                bounds: *key,
                background: if !self.clicked && self.selected == Some(i) {
                    Background::Color(Color::from_rgb8(0xA0, 0xA0, 0xA0))
                } else {
                    Background::Color(Color::WHITE)
                },
                border_radius: [0.5 / 70.0 * len; 4],
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
             });
        }
    }
}

impl<'a, Message, B, T> From<Macropad<'a, Message>> for Element<'a, Message, Renderer<B, T>>
where
    B: Backend,
    Message: 'a,
{
    fn from(macropad: Macropad<'a, Message>) -> Self {
        Self::new(macropad)
    }
}
