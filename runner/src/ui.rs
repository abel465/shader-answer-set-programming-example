use crate::{controller::Controller, fps_counter::FpsCounter, user_event::UserEvent};
use egui::{
    epaint::{textures::TexturesDelta, ClippedPrimitive},
    Context,
};
use egui_winit::{
    winit::{event::WindowEvent, event_loop::EventLoopProxy, window::Window},
    State,
};
use std::sync::Arc;

pub struct UiState {
    pub fps: usize,
    pub show_fps: bool,
    pub vsync: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            fps: 0,
            show_fps: true,
            vsync: true,
        }
    }
}

pub struct Ui {
    egui_winit_state: State,
    event_proxy: EventLoopProxy<UserEvent>,
    fps_counter: FpsCounter,
}

impl Ui {
    pub fn new(window: Arc<Window>, event_proxy: EventLoopProxy<UserEvent>) -> Self {
        let context = Context::default();
        let viewport_id = context.viewport_id();
        let egui_winit_state = State::new(
            context,
            viewport_id,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        Self {
            egui_winit_state,
            event_proxy,
            fps_counter: FpsCounter::new(),
        }
    }

    pub fn consumes_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        self.egui_winit_state
            .on_window_event(window, event)
            .consumed
    }

    pub fn prepare(
        &mut self,
        window: &Window,
        ui_state: &mut UiState,
        controller: &mut Controller,
    ) -> (Vec<ClippedPrimitive>, TexturesDelta) {
        ui_state.fps = self.fps_counter.tick();
        let raw_input = self.egui_winit_state.take_egui_input(window);
        let full_output = self.egui_winit_state.egui_ctx().run(raw_input, |ctx| {
            self.ui(ctx, ui_state, controller);
        });
        self.egui_winit_state
            .handle_platform_output(window, full_output.platform_output);
        let clipped_primitives = self
            .egui_winit_state
            .egui_ctx()
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        (clipped_primitives, full_output.textures_delta)
    }

    fn send_event(&self, event: UserEvent) {
        let _ = self.event_proxy.send_event(event);
    }

    fn ui(&self, ctx: &Context, ui_state: &mut UiState, controller: &mut Controller) {
        let resp = egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Settings", |ui| {
                    ui.checkbox(&mut ui_state.show_fps, "fps counter");
                    if ui.checkbox(&mut ui_state.vsync, "V-Sync").clicked() {
                        self.send_event(UserEvent::SetVSync(ui_state.vsync));
                    }
                });
                if ui_state.show_fps {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.style_mut().interaction.selectable_labels = false;
                        ui.label(format!("FPS: {}", ui_state.fps));
                    });
                }
            });
        });
        debug_assert_eq!(resp.response.rect.height(), shared::UI_MENU_HEIGHT as f32);
        let resp = egui::SidePanel::right("right_panel")
            .resizable(false)
            .show(ctx, |ui| {
                controller.ui(ctx, ui, &self.event_proxy);
            });
        debug_assert_eq!(resp.response.rect.width(), shared::UI_SIDEBAR_WIDTH as f32);
    }
}
