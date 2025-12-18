use eframe::egui;
use std::sync::Arc;
use std::time::{Duration, Instant};

// Window
const WINDOW_SIZE: [f32; 2] = [200.0, 200.0];

// Close button
const CLOSE_BTN_SIZE: f32 = 20.0;
const CLOSE_BTN_MARGIN: f32 = 8.0;
const CLOSE_BTN_PADDING: f32 = 4.0;
const CLOSE_BTN_STROKE: f32 = 2.0;

// Font sizes
const TIMER_FONT_SIZE: f32 = 52.0;
const INPUT_WIDTH: f32 = 80.0;

// Spacing
const INPUT_SPACING: f32 = 20.0;
const TIMER_BUTTON_SPACING: f32 = 30.0;

// Content heights for vertical centering
const INPUT_CONTENT_HEIGHT: f32 = 120.0;
const TIMER_CONTENT_HEIGHT: f32 = 52.0;

// Timing
const FLASH_INTERVAL_MS: u64 = 500;

// Colors
const BG_COLOR: (u8, u8, u8) = (30, 30, 30);
const FLASH_COLOR: (u8, u8, u8) = (255, 80, 80);
const CLOSE_BTN_COLOR: (u8, u8, u8) = (150, 150, 150);
const CLOSE_BTN_HOVER_COLOR: (u8, u8, u8) = (255, 100, 100);
const INFO_BTN_HOVER_COLOR: (u8, u8, u8) = (100, 150, 255);

// Links
const GITHUB_URL: &str = "https://github.com/kartikay-bagla/rust-gui-timer";

fn load_icon() -> Option<Arc<egui::IconData>> {
    let icon_bytes = include_bytes!("../icon.png");
    let image = image::load_from_memory(icon_bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    Some(Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }))
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size(WINDOW_SIZE)
        .with_title("Timer")
        .with_decorations(false)
        .with_transparent(true);

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Timer",
        options,
        Box::new(|_cc| Ok(Box::new(TimerApp::default()))),
    )
}

#[derive(PartialEq)]
enum TimerState {
    Input,
    Running,
    Finished,
}

struct TimerApp {
    minutes_input: String,
    state: TimerState,
    duration_secs: u64,
    remaining_secs: u64,
    start_time: Option<Instant>,
    flash_on: bool,
    last_flash: Instant,
    show_info: bool,
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            minutes_input: String::new(),
            state: TimerState::Input,
            duration_secs: 0,
            remaining_secs: 0,
            start_time: None,
            flash_on: false,
            last_flash: Instant::now(),
            show_info: false,
        }
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaints when timer is active
        if self.state == TimerState::Running || self.state == TimerState::Finished {
            ctx.request_repaint();
        }

        // Update timer logic
        if self.state == TimerState::Running {
            if let Some(start) = self.start_time {
                let elapsed = start.elapsed().as_secs();
                if elapsed >= self.duration_secs {
                    self.remaining_secs = 0;
                    self.state = TimerState::Finished;
                    self.last_flash = Instant::now();
                } else {
                    self.remaining_secs = self.duration_secs - elapsed;
                }
            }
        }

        // Flash logic when finished
        if self.state == TimerState::Finished {
            if self.last_flash.elapsed() >= Duration::from_millis(FLASH_INTERVAL_MS) {
                self.flash_on = !self.flash_on;
                self.last_flash = Instant::now();
            }
        }

        // Set background color based on state
        let bg_color = if self.state == TimerState::Finished && self.flash_on {
            egui::Color32::from_rgb(FLASH_COLOR.0, FLASH_COLOR.1, FLASH_COLOR.2)
        } else {
            egui::Color32::from_rgb(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2)
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(bg_color))
            .show(ctx, |ui| {
                // Title bar region for dragging (same height as close button area)
                let title_bar_height = CLOSE_BTN_SIZE + CLOSE_BTN_MARGIN * 2.0;
                let close_btn_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        ui.max_rect().right() - CLOSE_BTN_SIZE - CLOSE_BTN_MARGIN,
                        ui.max_rect().top() + CLOSE_BTN_MARGIN,
                    ),
                    egui::vec2(CLOSE_BTN_SIZE, CLOSE_BTN_SIZE),
                );
                let info_btn_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        ui.max_rect().right() - (CLOSE_BTN_SIZE + CLOSE_BTN_MARGIN) * 2.0,
                        ui.max_rect().top() + CLOSE_BTN_MARGIN,
                    ),
                    egui::vec2(CLOSE_BTN_SIZE, CLOSE_BTN_SIZE),
                );
                let title_bar_rect = egui::Rect::from_min_size(
                    ui.max_rect().left_top(),
                    egui::vec2(ui.max_rect().width() - (CLOSE_BTN_SIZE + CLOSE_BTN_MARGIN) * 2.0 - CLOSE_BTN_MARGIN, title_bar_height),
                );

                // Drag window from title bar region (excluding buttons)
                if ui.rect_contains_pointer(title_bar_rect) && ui.input(|i| i.pointer.primary_pressed()) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                // Draw info button (i)
                let info_hovered = ui.rect_contains_pointer(info_btn_rect);
                let info_clicked = info_hovered && ui.input(|i| i.pointer.primary_clicked());
                let info_color = if info_hovered {
                    egui::Color32::from_rgb(INFO_BTN_HOVER_COLOR.0, INFO_BTN_HOVER_COLOR.1, INFO_BTN_HOVER_COLOR.2)
                } else {
                    egui::Color32::from_rgb(CLOSE_BTN_COLOR.0, CLOSE_BTN_COLOR.1, CLOSE_BTN_COLOR.2)
                };
                let info_center = info_btn_rect.center();
                ui.painter().text(
                    info_center,
                    egui::Align2::CENTER_CENTER,
                    "i",
                    egui::FontId::proportional(14.0),
                    info_color,
                );
                if info_clicked {
                    self.show_info = !self.show_info;
                }

                // Draw close button (x)
                let close_hovered = ui.rect_contains_pointer(close_btn_rect);
                let close_clicked = close_hovered && ui.input(|i| i.pointer.primary_clicked());
                let close_color = if close_hovered {
                    egui::Color32::from_rgb(CLOSE_BTN_HOVER_COLOR.0, CLOSE_BTN_HOVER_COLOR.1, CLOSE_BTN_HOVER_COLOR.2)
                } else {
                    egui::Color32::from_rgb(CLOSE_BTN_COLOR.0, CLOSE_BTN_COLOR.1, CLOSE_BTN_COLOR.2)
                };
                ui.painter().line_segment(
                    [close_btn_rect.left_top() + egui::vec2(CLOSE_BTN_PADDING, CLOSE_BTN_PADDING), close_btn_rect.right_bottom() - egui::vec2(CLOSE_BTN_PADDING, CLOSE_BTN_PADDING)],
                    egui::Stroke::new(CLOSE_BTN_STROKE, close_color),
                );
                ui.painter().line_segment(
                    [close_btn_rect.right_top() + egui::vec2(-CLOSE_BTN_PADDING, CLOSE_BTN_PADDING), close_btn_rect.left_bottom() + egui::vec2(CLOSE_BTN_PADDING, -CLOSE_BTN_PADDING)],
                    egui::Stroke::new(CLOSE_BTN_STROKE, close_color),
                );
                if close_clicked {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                // Info popup
                if self.show_info {
                    egui::Window::new("About")
                        .collapsible(false)
                        .resizable(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .show(ctx, |ui| {
                            ui.label("Desktop Timer");
                            ui.label("Licensed under GPL v3");
                            ui.add_space(10.0);
                            if ui.button("View on GitHub").clicked() {
                                let _ = open::that(GITHUB_URL);
                                self.show_info = false;
                            }
                            ui.add_space(10.0);
                            if ui.button("Close").clicked() {
                                self.show_info = false;
                            }
                        });
                }

                ui.vertical_centered(|ui| {
                    // Estimate content heights for vertical centering
                    let content_height = match self.state {
                        TimerState::Input => INPUT_CONTENT_HEIGHT,
                        TimerState::Running => TIMER_CONTENT_HEIGHT,
                        TimerState::Finished => TIMER_CONTENT_HEIGHT,
                    };
                    let available_height = ui.available_height();
                    let top_padding = (available_height - content_height) / 2.0;
                    ui.add_space(top_padding.max(0.0));

                    match self.state {
                        TimerState::Input => {
                            ui.add(egui::Label::new(egui::RichText::new("Set Timer").heading()).selectable(false));
                            ui.add_space(INPUT_SPACING);

                            let input = ui.add(
                                egui::TextEdit::singleline(&mut self.minutes_input)
                                    .hint_text("minutes")
                                    .desired_width(INPUT_WIDTH)
                                    .font(egui::TextStyle::Heading),
                            );

                            // Start on Enter key
                            if input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                self.try_start();
                            }

                            ui.add_space(INPUT_SPACING);

                            if ui.button("Start").clicked() {
                                self.try_start();
                            }
                        }

                        TimerState::Running => {
                            let time_str = format_time(self.remaining_secs);
                            ui.add(egui::Label::new(
                                egui::RichText::new(time_str)
                                    .size(TIMER_FONT_SIZE)
                                    .color(egui::Color32::WHITE)
                                    .monospace(),
                            ).selectable(false));
                            ui.add_space(TIMER_BUTTON_SPACING);

                            if ui.button("Cancel").clicked() {
                                self.reset();
                            }
                        }

                        TimerState::Finished => {
                            let text_color = if self.flash_on {
                                egui::Color32::WHITE
                            } else {
                                egui::Color32::from_rgb(FLASH_COLOR.0, FLASH_COLOR.1, FLASH_COLOR.2)
                            };

                            ui.add(egui::Label::new(
                                egui::RichText::new("00:00")
                                    .size(TIMER_FONT_SIZE)
                                    .color(text_color)
                                    .monospace(),
                            ).selectable(false));
                            ui.add_space(TIMER_BUTTON_SPACING);

                            if ui.button("Reset").clicked() {
                                self.reset();
                            }
                        }
                    }
                });
            });
    }
}

impl TimerApp {
    fn try_start(&mut self) {
        if let Ok(mins) = self.minutes_input.trim().parse::<f64>() {
            if mins > 0.0 {
                self.duration_secs = (mins * 60.0) as u64;
                self.remaining_secs = self.duration_secs;
                self.start_time = Some(Instant::now());
                self.state = TimerState::Running;
            }
        }
    }

    fn reset(&mut self) {
        self.state = TimerState::Input;
        self.minutes_input.clear();
        self.start_time = None;
        self.flash_on = false;
    }
}

fn format_time(secs: u64) -> String {
    let m = secs / 60;
    let s = secs % 60;
    format!("{:02}:{:02}", m, s)
}
