mod cat;
mod color;
mod race;
mod log_color;

use eframe::egui;
use eframe::egui::{Align, Layout};
use crate::cat::CatInfo;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 1200.0]),
        ..Default::default()
    };
    let mut cats = CatInfo::spawn_new_cat(4);
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

            // print image
            //egui::ScrollArea::both().show(ui, |ui| {
            //    ui.add(egui::Image::from_uri("https://w7.pngwing.com/pngs/174/600/png-transparent-cat-animal-lovely-cat.png").rounding(10.0), );
            //});

            self.columns = 0;

            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.columns(5, |columns| {
                    for cat in &self.cat_vec {
                        columns[self.columns].group(|ui| {

                            ui.add(egui::ImageButton::new("https://images.apilist.fun/the_cat_api_api.png").rounding(10.0));
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
            })
        });

        // close confirmation
        ask_close_app(self, &ctx);
    }
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