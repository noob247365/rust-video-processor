use eframe::egui;

#[derive(Default)]
struct MyApp {
    counter: u32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |_ui| {});
        
        let centered_area_id = ui.scope_id().with("centered_area");
        egui::Area::new(centered_area_id)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(),|ui| {
                let button = egui::Button::new(format!("Clicked {} times", self.counter))
                    .wrap_mode(egui::TextWrapMode::Extend);
                if ui.add(button).clicked() {
                    self.counter += 1;
                }
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 150.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My first egui app",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
