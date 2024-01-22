#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epaint::{vec2, TextureHandle}};
use egui_plot::{Plot, PlotImage, PlotPoint};
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Plot fold reproducer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    texture: Option<TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            texture: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|s| s.animation_time = 3.0);
        egui::Window::new("Plot fold reporoducer").show(ctx, |ui| {
            Plot::new("My Plot").show(ui, |plot_ui| {
                let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                    plot_ui.ctx().load_texture(
                        "plot_demo",
                        egui::ColorImage::example(),
                        Default::default(),
                    )
                });
                let image = PlotImage::new(
                    texture,
                    PlotPoint::new(0.0, 10.0),
                    5.0 * vec2(texture.aspect_ratio(), 1.0),
                );
                plot_ui.image(image.name("Image"));
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
           
        });
    }
}
