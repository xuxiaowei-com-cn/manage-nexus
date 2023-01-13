// https://github.com/emilk/egui/blob/0.19.0/examples/hello_world/src/main.rs
// https://github.com/emilk/egui/blob/0.19.0/examples/custom_font/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 在版本中隐藏Windows上的控制台窗口


use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "管理 nexus 仓库",
        options,
        Box::new(|cc| Box::new(Input::new(cc))),
    );
}

struct Input {
    heading: String,
    username: String,
    password: String,
    url: String,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            heading: "上传 Maven 依赖".to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
            url: "".to_owned(),
        }
    }
}

impl Input {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
}


impl eframe::App for Input {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&mut self.heading);
            ui.horizontal(|ui| {
                ui.label("用户名: ");
                ui.text_edit_singleline(&mut self.username);
            });
            ui.horizontal(|ui| {
                ui.label("密码: ");
                ui.text_edit_singleline(&mut self.password);
            });
            ui.horizontal(|ui| {
                ui.label("地址: ");
                ui.text_edit_singleline(&mut self.url);
            });
            if ui.button("上传").clicked() {}
        });
    }
}

// https://github.com/emilk/egui/blob/0.19.0/examples/custom_font/src/main.rs
fn setup_custom_fonts(ctx: &egui::Context) {
    // 从默认字体开始（我们将添加到它们，而不是替换它们）。
    let mut fonts = egui::FontDefinitions::default();

    // 安装我自己的字体（可能支持非拉丁字符）。
    // 支持.ttf和.otf文件。
    fonts.font_data.insert(
        "my_font".to_owned(),
        // 得意黑字体 SmileySans-Oblique.otf 来自 https://github.com/atelier-anchor/smiley-sans
        egui::FontData::from_static(include_bytes!("fonts/SmileySans-Oblique.otf")),
    );

    // 将我的字体放在比例文本的第一位（最高优先级）：
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // 将我的字体作为monospace的最后备用字体：
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // 告诉egui使用这些字体：
    ctx.set_fonts(fonts);
}
