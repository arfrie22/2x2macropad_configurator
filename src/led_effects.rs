use std::time::Instant;

use iced::Color;

use crate::macro_parser::LedConfig;

#[derive(Debug, Clone, Copy)]
pub struct LedRunner {
    timer: f32,
    last_update: Instant,
}

impl Default for LedRunner {
    fn default() -> Self {
        Self {
            timer: 0.0,
            last_update: Instant::now(),
        }
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

impl LedRunner {
    pub fn get_leds(&self, config: &LedConfig) -> [Color; 4] {
        let effect = config.effect.unwrap_or(macropad_protocol::data_protocol::LedEffect::None);
        let base_color = config.base_color.unwrap_or_default();
        let brightness = config.brightness.unwrap_or_default();
        let effect_period = config.effect_period.unwrap_or_default();

        match &effect {
            macropad_protocol::data_protocol::LedEffect::None => [Color::TRANSPARENT; 4],
            macropad_protocol::data_protocol::LedEffect::Static => {
                [Color::from_rgba8(
                    base_color.0,
                    base_color.1,
                    base_color.2,
                    brightness as f32 / 255.0,
                ); 4]
            }
            macropad_protocol::data_protocol::LedEffect::Breathing => {
                let color = base_color;

                let mut time = self.timer * (100.0 / 1000.0);

                if time > 50.0 {
                    time = 100.0 - time;
                }

                [Color::from_rgba8(
                    color.0,
                    color.1,
                    color.2,
                    (time / 50.0) * (brightness as f32 / 255.0),
                ); 4]
            }
            macropad_protocol::data_protocol::LedEffect::BreathingSpaced => {
                let color = base_color;
                let timer = self.timer * (400.0 / 1000.0);

                let mut backlight = [Color::BLACK; 4];

                for (index, led) in backlight.iter_mut().enumerate() {
                    let mut time = timer;
                    time -= index as f32 * 100.0;

                    if !(0.0..=100.0).contains(&time) {
                        *led = Color::TRANSPARENT;
                        continue;
                    }

                    if time > 50.0 {
                        time = 100.0 - time;
                    }

                    *led = Color::from_rgba8(
                        color.0,
                        color.1,
                        color.2,
                        (time / 50.0) * (brightness as f32 / 255.0),
                    );
                }

                backlight
            }
            macropad_protocol::data_protocol::LedEffect::ColorCycle => {
                let timer = self.timer * (360.0 / 1000.0);
                let color = hsv2rgb(timer, 1.0, 1.0);

                [Color::from_rgba(color.0, color.1, color.2, brightness as f32 / 255.0); 4]
            }
            macropad_protocol::data_protocol::LedEffect::Rainbow => {
                let timer = self.timer * (360.0 / 1000.0);
                let mut backlight = [Color::BLACK; 4];

                for (index, led) in backlight.iter_mut().enumerate() {
                    let color = hsv2rgb((timer + (index as f32 * 360.0 / 4.0)) % 360.0, 1.0, 1.0);
                    *led = Color::from_rgba(
                        color.0,
                        color.1,
                        color.2,
                        brightness as f32 / 255.0,
                    );
                }

                backlight
            }
        }
    }

    pub fn update(&mut self, config: &LedConfig) {
        if config.effect_period.unwrap_or_default() != 0.0 {
            self.timer += (1.0 / config.effect_period.unwrap_or_default())
                * ((Instant::now().duration_since(self.last_update).as_micros()) as f32 / 1000.0);

            self.timer = self.timer % 1000.0;

            if self.timer < 0.0 {
                self.timer += 1000.0;
            }

            self.last_update = Instant::now();
        }
    }

    pub fn reset(&mut self) {
        self.timer = 0.0;
        self.last_update = Instant::now();
    }
}
