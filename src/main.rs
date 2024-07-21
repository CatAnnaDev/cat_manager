mod cat;
mod color;
mod race;
mod log_color;

use eframe::egui;
use eframe::egui::{Align, Align2, Button, Layout};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use crate::cat::CatInfo;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 1200.0]),
        ..Default::default()
    };
    let cats = CatInfo::spawn_new_cat(2);
    eframe::run_native(
        "Cat Manager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let data = MyApp {
                show_confirmation_dialog: false,
                allowed_to_close: false,
                columns: 0,
                cat_vec: cats,
            };
            Ok(Box::new(data))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    columns: usize,
    cat_vec: Vec<CatInfo>
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut toasts = Toasts::new().anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)).direction(egui::Direction::BottomUp);
            if ui.add(Button::new("Spawn new cat")).clicked(){
                self.cat_vec.push(CatInfo::new_cat());
            }
            self.columns = 0;

            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.columns(5, |columns| {
                        for cat in self.cat_vec.iter_mut() {
                            columns[self.columns].group(|ui| {
                                ui.add(egui::Image::new("https://images.apilist.fun/the_cat_api_api.png").rounding(10.0)).context_menu(|ui| {
                                    if ui.add(Button::new("Feed")).clicked() { toast(&mut toasts, cat.feed()); }
                                    if ui.add(Button::new("Play")).clicked() { toast(&mut toasts, cat.play()); }
                                    if ui.add(Button::new("Sleep")).clicked() { toast(&mut toasts, cat.toggle_sleep()); }
                                });

                                ui.add(egui::Label::new(format!("{}", cat)).truncate());

                                if self.columns == 4 {
                                    ui.end_row();
                                    self.columns = 0;
                                } else {
                                    self.columns += 1;
                                }
                            });
                        }
                    })
                });
            });
            // update ui toast
            toasts.show(ctx);

        });
        // close confirmation
        ask_close_app(self, &ctx);
    }
}

fn toast(toasts: &mut Toasts, message: String){
    toasts.add(Toast {
        text: message.into(),
        kind: ToastKind::Success,
        options: ToastOptions::default()
            .duration_in_seconds(2.0)
            .show_progress(true),
        ..Default::default()
    });
}

fn ask_close_app(data_self: &mut MyApp, ctx: &egui::Context) {
    if ctx.input(|i| i.viewport().close_requested()) {
        if data_self.allowed_to_close {} else {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            data_self.show_confirmation_dialog = true;
        }
    }
    if data_self.show_confirmation_dialog {
        egui::Window::new("Do you want to quit?")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("No").clicked() {
                        data_self.show_confirmation_dialog = false;
                        data_self.allowed_to_close = false;
                    }

                    if ui.button("Yes").clicked() {
                        data_self.show_confirmation_dialog = false;
                        data_self.allowed_to_close = true;
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
    }
}