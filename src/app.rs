use eframe::egui;
use sf6_rsz_parser::rsz::{RSZData,RSZField,RSZValue};
use sf6_rsz_parser::rsz::json_parser::{TypeIDs};
#[derive(Default)]
pub struct Editor{
    name: String,
}
impl Editor{
    pub(crate) fn new(cc:&eframe::CreationContext<'_>)->Self{
        Self { name: "Editor".to_owned() }
    }
}

impl eframe::App for Editor{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let testdata = RSZField{
            name:String::from("Test Field Name"),
            value_type: TypeIDs::String,
            value:RSZValue::String(String::from("Test string")),
            alignment: 4,
        };
        egui::CentralPanel::default().show(ctx,|ui| {
            ui.heading(&self.name);
        });
    }
}