use eframe::egui;
use sf6_rsz_parser::rsz::{RSZData,RSZField,RSZValue,Int2,Float2,UInt3,Float4};
use sf6_rsz_parser::rsz::json_parser::{TypeIDs};
mod rsz_egui;
#[derive(Default)]
pub struct Editor{
    name: String,
    //there should just be a holder for the whole file structure here but for the moment fuck it we Vec
    pub fields: Vec<RSZField>,
}
impl Editor{
    pub(crate) fn new(cc:&eframe::CreationContext<'_>)->Self{
        Self { 
            name: "Editor".to_owned(),
            //we're defining a test RSZField of type "String" here for testing purposes
            fields: vec![
                RSZField{
                name:String::from("Test Field Name"),
                value_type: TypeIDs::String,
                value:RSZValue::String(String::from("Test string")),
                alignment: 4,
                },
                RSZField{
                    name:String::from("Test Boolean value"),
                    value_type: TypeIDs::Bool,
                    value:RSZValue::Bool(false),
                    alignment: 4,
                },
                RSZField{
                    name:String::from("Test Boolean value"),
                    value_type: TypeIDs::Bool,
                    value:RSZValue::Bool(false),
                    alignment: 4,
                },
                RSZField{
                    name:String::from("Test Third boolean value"),
                    value_type: TypeIDs::Bool,
                    value:RSZValue::Bool(false),
                    alignment: 4,
                },
                RSZField{
                    name:String::from("Float"),
                    value_type: TypeIDs::F32,
                    value:RSZValue::Float(1.0),
                    alignment: 4,
                },
                RSZField{
                    name:String::from("Double"),
                    value_type: TypeIDs::F64,
                    value:RSZValue::Double(1.0),
                    alignment: 8,
                },
                RSZField{
                    name:String::from("Coordinates"),
                    value_type: TypeIDs::Float2,
                    value:RSZValue::Float2(Float2{x:2.5,y:5.6}),
                    alignment: 4,
                },
                RSZField{
                    name:String::from("Int2"),
                    value_type: TypeIDs::Int2,
                    value:RSZValue::Int2(Int2{x:3,y:5}),
                    alignment: 8,
                },
                RSZField{
                    name:String::from("Uint3"),
                    value_type: TypeIDs::Uint3,
                    value:RSZValue::UInt3(UInt3{x:3,y:5,z:8}),
                    alignment: 8,
                },
                RSZField{
                    name:String::from("Rotation"),
                    value_type: TypeIDs::Float4,
                    value:RSZValue::Float4(Float4{x:3.0,y:5.0,z:8.1,w:1.0}),
                    alignment: 8,
                },
            ],
        }
    }
}
//this makes the editor an egui application, and implements its ui repaint function, where the UI values go.
impl eframe::App for Editor{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        //we define the UI function inline here after passing the context. 
        //Presumably you could declare the funciton elsewhere and it'd be fine.  
        egui::CentralPanel::default().show(ctx,|ui| {
            //add the title of the window? Actually, this is just a header field, maybe this should be the object name?
            ui.heading(&self.name);
            //we setup a grid here so that the labels and the values can be evenly spaced.
            egui::Grid::new("Editor")
                .num_columns(2)
                .striped(true)
                .show(ui,|ui|{
                    //this creates a MUTABLE iterator, 
                    //which acts as a mutable reference to the first RSZField in our vector, allowing us to edit it.
                    //the for loop here takes control of the iterator and manually calls iter.next, which gets the next RSZField.
                    let field_iter = self.fields.iter_mut();
                    for field in field_iter{
                        //we then pass a mutable reference to both the RSZField and the UI to display_field, 
                        //which renders the Field in our editor.
                        rsz_egui::display_field(field,ui);
                    };
                });
            
            

        });
    }
}