use crate::{
    bind_group_buffer::{BindGroupBufferType, BufferData, SSBO},
    user_event::UserEvent,
    Options,
};
use bytemuck::Zeroable;
use egui::Context;
use egui_winit::winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
    event_loop::EventLoopProxy,
    keyboard::{Key, NamedKey},
};
use glam::*;
use shared::push_constants::shader::*;
use shared::{UI_MENU_HEIGHT, UI_SIDEBAR_WIDTH};
use std::time::Instant;

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    fragment_constants: FragmentConstants,
    compute_constants: ComputeConstants,
    cursor: Vec2,
    prev_cursor: Vec2,
    mouse_button_pressed: u32,
    speed: f32,
    distance: f32,
    last_frame: Instant,
    zoom: f32,
    debug: bool,
    buffer: Vec<f32>,
}

impl Controller {
    pub fn new(size: PhysicalSize<u32>, options: &Options) -> Self {
        let now = Instant::now();
        let size = PhysicalSize {
            width: size.width - UI_SIDEBAR_WIDTH,
            height: size.height - UI_MENU_HEIGHT,
        };

        let debug = options.debug;
        let speed = normalize_speed_down(!debug as u32 as f32);

        Self {
            size,
            start: now,
            fragment_constants: FragmentConstants::zeroed(),
            compute_constants: ComputeConstants::zeroed(),
            cursor: Vec2::ZERO,
            prev_cursor: Vec2::ZERO,
            mouse_button_pressed: 0,
            speed,
            distance: 0.0,
            last_frame: now,
            zoom: 1.0,
            debug,
            buffer: vec![0.0],
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = PhysicalSize {
            width: size.width - UI_SIDEBAR_WIDTH,
            height: size.height - UI_MENU_HEIGHT,
        };
    }

    pub fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32 - UI_MENU_HEIGHT as f32);
    }

    pub fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        let val = match delta {
            MouseScrollDelta::LineDelta(_, val) => val * 0.1,
            MouseScrollDelta::PixelDelta(p) => (p.y * 0.005) as f32,
        };
        self.zoom = (self.zoom + self.zoom * val).clamp(1.0, 100.0);
    }

    pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        let mask = 1
            << match button {
                MouseButton::Left => 0,
                MouseButton::Middle => 1,
                MouseButton::Right => 2,
                MouseButton::Back => 3,
                MouseButton::Forward => 4,
                MouseButton::Other(i) => 5 + (i as usize),
            };
        match state {
            ElementState::Pressed => self.mouse_button_pressed |= mask,
            ElementState::Released => self.mouse_button_pressed &= !mask,
        }
    }

    pub fn keyboard_input(&mut self, key: KeyEvent) {
        match key.logical_key {
            Key::Character(c) => match c.chars().next().unwrap() {
                'z' => {}
                'x' => {}
                _ => {}
            },
            Key::Named(NamedKey::ArrowLeft) => {}
            _ => {}
        }
    }

    pub fn pre_render(&mut self) {
        self.fragment_constants = FragmentConstants {
            size: self.size.into(),
            time: self.start.elapsed().as_secs_f32(),
            mouse_button_pressed: self.mouse_button_pressed,
            cursor: self.cursor.into(),
            prev_cursor: self.prev_cursor.into(),
            zoom: self.zoom,
            debug: self.debug.into(),
        };
        self.prev_cursor = self.cursor;
    }

    pub fn pre_update(&mut self) {
        self.compute_constants = ComputeConstants {
            size: self.size.into(),
            time: self.start.elapsed().as_secs_f32(),
            zoom: self.zoom,
        };
    }

    pub fn fragment_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.fragment_constants)
    }

    pub fn compute_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.compute_constants)
    }

    pub fn ui(
        &mut self,
        _ctx: &Context,
        ui: &mut egui::Ui,
        _event_proxy: &EventLoopProxy<UserEvent>,
    ) {
        ui.add(egui::Label::new(" Simulation Speed").selectable(false));
        ui.add(
            egui::Slider::new(&mut self.speed, 0.0..=1.99)
                .custom_formatter(|x, _| format!("{:.2}", normalize_speed_up(x as f32)))
                .custom_parser(|x| x.parse().map(|x: f32| normalize_speed_down(x) as f64).ok()),
        );
        ui.add(egui::Label::new("           Zoom").selectable(false));
        ui.add(
            egui::Slider::new(&mut self.zoom, 1.0..=100.0)
                .logarithmic(true)
                .max_decimals(2),
        );
        ui.checkbox(&mut self.debug, "Debug");
        if self.debug {
            ui.label(format!(
                "Elapsed: {:.1}s",
                self.start.elapsed().as_secs_f64()
            ));
        }
    }

    pub fn buffers(&self) -> BufferData {
        BufferData {
            bind_group_buffers: vec![BindGroupBufferType::SSBO(SSBO {
                data: bytemuck::cast_slice(&self.buffer[..]),
                read_only: false,
            })],
        }
    }

    pub fn iterations(&mut self) -> u32 {
        let speed = normalize_speed_up(self.speed);
        let t = self.last_frame.elapsed().as_secs_f32() * 100.0;
        self.last_frame = Instant::now();
        self.distance += speed * t;
        if self.distance >= 1.0 {
            let iterations = self.distance as u32;
            self.distance = self.distance.fract();
            iterations
        } else {
            0
        }
    }
}

fn normalize_speed_down(x: f32) -> f32 {
    (x / 25.0).sqrt()
}

fn normalize_speed_up(x: f32) -> f32 {
    x * x * 25.0
}
