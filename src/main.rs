use eframe::egui;
use eframe::egui::{Align2, Button};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

use crate::cat::CatInfo;

mod cat;
mod color;
mod race;
mod log_color;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 1000.0]),
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
    cat_vec: Vec<CatInfo>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut toasts = Toasts::new().anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)).direction(egui::Direction::BottomUp);
            if ui.add(Button::new("Spawn new cat")).clicked() {
                self.cat_vec.push(CatInfo::new_cat());
            }
            self.columns = 0;

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.columns(8, |columns| {
                    for cat in 0..self.cat_vec.len() {
                        columns[self.columns].group(|ui| {
                            let img = self.cat_vec[cat].cat_image_byte.clone();
                            println!("{img}");
                            ui.add(egui::Image::new(format!("file://{img}")).rounding(10.0)).context_menu(|ui| {
                                if ui.add(Button::new("Feed")).clicked() { toast(&mut toasts, self.cat_vec[cat].feed()); }
                                if ui.add(Button::new("Play")).clicked() { toast(&mut toasts, self.cat_vec[cat].play()); }
                                if ui.add(Button::new("Sleep")).clicked() { toast(&mut toasts, self.cat_vec[cat].toggle_sleep()); }
                                ui.menu_button("Mate with", |ui| {
                                    for mate_cat in 0..self.cat_vec.len() {
                                        if self.cat_vec[cat].gender.ne(&self.cat_vec[mate_cat].gender) {
                                            if ui.button(self.cat_vec[mate_cat].name).clicked() {
                                                match self.cat_vec[cat].mate(&self.cat_vec[mate_cat]) {
                                                    Ok(e) => { self.cat_vec.push(e) }
                                                    Err(e) => { toast(&mut toasts, e) }
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                            //ui.ctx().forget_all_images();
                            ui.add(egui::Label::new(format!("{}", self.cat_vec[cat])).truncate());
                        });

                        if self.columns == 7 {
                            self.columns = 0;
                        } else {
                            self.columns += 1;
                        }
                    }


                })
            });

            // update ui toast
            toasts.show(ctx);
        });
        // close confirmation
        ask_close_app(self, &ctx);
    }
}


fn toast(toasts: &mut Toasts, message: String) {
    toasts.add(Toast {
        text: message.into(),
        kind: ToastKind::Success,
        options: ToastOptions::default()
            .duration_in_seconds(5.0)
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