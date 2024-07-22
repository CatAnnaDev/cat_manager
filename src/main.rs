use eframe::egui;
use eframe::egui::{Align2, Button};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use std::time::{Duration, Instant};

use crate::cat::CatInfo;

mod cat;
mod color;
mod race;
mod log_color;
mod cat_name;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 1000.0]),
        vsync: true,
        ..Default::default()
    };

    let cats = CatInfo::spawn_new_cat(2);
    eframe::run_native(
        "Cat Manager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp {
                show_confirmation_dialog: false,
                allowed_to_close: false,
                columns: 0,
                cat_vec: cats,
                last_update: Instant::now(),
            }))
        }),
    )
}

struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    columns: usize,
    cat_vec: Vec<CatInfo>,
    last_update: Instant,
}

impl MyApp {
    fn handle_cats_update(&mut self, toasts: &mut Toasts) {
        if self.last_update.elapsed() >= Duration::from_secs(30) {
            let mut rm_cat = vec![];
            for index in 0..self.cat_vec.len() {
                if let None = self.cat_vec[index].update() {
                    rm_cat.push(index);
                    die_cat_toast(toasts, format!("{}", self.cat_vec[index]));
                }
                toast(toasts, self.cat_vec[index].minimal_info());
            }
            self.last_update = Instant::now();

            for x in rm_cat.iter().rev() {
                self.cat_vec.remove(*x);
            }
        }
    }

    fn show_cat_actions(&mut self, ui: &mut egui::Ui, toasts: &mut Toasts, cat: &mut CatInfo) {
        if ui.add(Button::new("Feed")).clicked() {
            toast(toasts, cat.feed(0.1, 5.0));
            ui.close_menu();
        }
        if ui.add(Button::new("Play")).clicked() {
            toast(toasts, cat.play(0.05, 2.0));
            ui.close_menu();
        }
        if ui.add(Button::new(format!("Sleep ({})", bool_state!("YES", "NO", cat.sleep)))).clicked() {
            toast(toasts, cat.toggle_sleep(10.0));
            ui.close_menu();
        }

        ui.menu_button("Mate with", |ui| {
            for mate_cat in 0..self.cat_vec.len() {
                if cat.gender.ne(&self.cat_vec[mate_cat].gender) && ui.button(self.cat_vec[mate_cat].name).clicked() {
                    match cat.mate(&self.cat_vec[mate_cat]) {
                        Ok(kitten) => {
                            self.cat_vec.push(kitten);
                            ui.close_menu();
                        }
                        Err(e) => toast(toasts, e),
                    }
                }
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("A");
        let mut toasts = Toasts::new().anchor(Align2::RIGHT_TOP, (-10.0, 10.0)).direction(egui::Direction::TopDown);
        self.handle_cats_update(&mut toasts);

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.add(Button::new("Spawn new cat")).clicked() {
                self.cat_vec.push(CatInfo::new_cat());
            }

            if ui.add(Button::new("Cat age 50+")).clicked() {
                for x in 0..self.cat_vec.len() {
                    self.cat_vec[x].age += 50;
                }
            }

            self.columns = 0;
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.columns(8, |columns| {
                    for cat in 0..self.cat_vec.len() {
                        columns[self.columns].group(|ui| {
                            let image = ui.add(egui::Image::new(format!("file://{}", self.cat_vec[cat].cat_image_byte.clone())).rounding(10.0));
                            image.context_menu(|ui| self.show_cat_actions(ui, &mut toasts, &mut self.cat_vec[cat].clone()));

                            if image.hovered() {
                                ui.add(egui::Label::new(format!("{}", self.cat_vec[cat])));
                            } else {
                                ui.add(egui::Label::new(format!("{}", self.cat_vec[cat].minimal_info())));
                            }
                        });

                        self.columns = (self.columns + 1) % 8;
                    }
                });
            });

            toasts.show(ctx);
        });

        ask_close_app(self, ctx);
        ctx.request_repaint();
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

fn die_cat_toast(toasts: &mut Toasts, message: String) {
    toasts.add(Toast {
        text: message.into(),
        kind: ToastKind::Error,
        options: ToastOptions::default()
            .duration_in_seconds(20.0)
            .show_progress(true),
        ..Default::default()
    });
}

fn ask_close_app(app: &mut MyApp, ctx: &egui::Context) {
    if ctx.input(|i| i.viewport().close_requested()) && !app.allowed_to_close {
        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        app.show_confirmation_dialog = true;
    }

    if app.show_confirmation_dialog {
        egui::Window::new("Do you want to quit?")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("No").clicked() {
                        app.show_confirmation_dialog = false;
                    }

                    if ui.button("Yes").clicked() {
                        app.show_confirmation_dialog = false;
                        app.allowed_to_close = true;
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
    }
}
