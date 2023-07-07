use eframe::emath::Numeric;
use sf6_rsz_parser::rsz::{RSZField,RSZValue}; 
use sf6_rsz_parser::rsz::json_parser::{TypeIDs};
use eframe::egui;

pub fn plane_xz<T:Numeric>(ui:&mut egui::Ui,x:&mut T,z:&mut T){
    ui.horizontal(|ui|{
        ui.label("x");
        ui.add(egui::DragValue::new(x));
        ui.label("z");
        ui.add(egui::DragValue::new(z));
    });
}

pub fn drag2<T:Numeric>(ui:&mut egui::Ui,x:&mut T,y:&mut T){
    ui.horizontal(|ui|{
        ui.label("x");
        ui.add(egui::DragValue::new(x));
        ui.label("y");
        ui.add(egui::DragValue::new(y));
    });
}

pub fn drag3<T:Numeric>(ui:&mut egui::Ui,x:&mut T,y:&mut T,z:&mut T){
    ui.horizontal(|ui|{
        ui.label("x");
        ui.add(egui::DragValue::new(x));
        ui.label("y");
        ui.add(egui::DragValue::new(y));
        ui.label("z");
        ui.add(egui::DragValue::new(z));
        
    });
}
pub fn drag4<T:Numeric>(ui:&mut egui::Ui,x:&mut T,y:&mut T,z:&mut T,w:&mut T){
    ui.horizontal(|ui|{
        ui.label("x");
        ui.add(egui::DragValue::new(x));
        ui.label("y");
        ui.add(egui::DragValue::new(y));
        ui.label("z");
        ui.add(egui::DragValue::new(z));
        ui.label("w");
        ui.add(egui::DragValue::new(w));
    });
}


pub fn display_field(field:&mut RSZField,ui:&mut egui::Ui){
    //draw the field name in the first column
    ui.label(field.name.to_owned());
    //we use a match control flow here because data is stored as an RSZValue,
    //which is an enum.
    match field.value{
        //to match the enum correctly while being able to edit the value inside,
        //we use the keyword ref, which binds the value by reference, allowing the ui to edit it
        //(I don't know exactly what this does, fix this comment later lmao)
        RSZValue::Bool(ref mut value) => {
            //don't think this should be necessary, 
            //I was just trying to check we were updating the actual value as opposed to accidently creating a copy
            //or accessing in some other read-only manner.
            
            //ui.label(String::from(value.to_string()));

            //not sure if need a unique label for checkboxes like imgui, we'll see I guess.
            ui.checkbox( value,"");
        }
        RSZValue::Float(ref mut f)=>{ui.add(egui::DragValue::new(f).speed(1.0));},
        RSZValue::Double(ref mut d) =>{ui.add(egui::DragValue::new(d).speed(1.0));},

        RSZValue::PlaneXZ(ref mut num2)=>{
            plane_xz(ui, &mut num2.x,&mut num2.z);
        },
        RSZValue::Float2(ref mut num2)=>{
            drag2(ui, &mut num2.x,&mut num2.y);
        },
        RSZValue::Float3(ref mut num3)=>{
            drag3(ui, &mut num3.x,&mut num3.y,&mut num3.z);
        },
        RSZValue::Float4(ref mut num4)=>{
            drag4(ui, &mut num4.x,&mut num4.y,&mut num4.z,&mut num4.w);
        },
        //think this will need something more but fuck it.
        RSZValue::Fixed(ref mut f)=>{ui.add(egui::DragValue::new(f).speed(1.0));},

        //GUID

        RSZValue::Int8(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},
        RSZValue::Int16(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},
        RSZValue::Int32(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},
        RSZValue::Int64(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},
        RSZValue::UInt8(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},
        RSZValue::UInt16(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},
        RSZValue::UInt32(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},
        RSZValue::UInt64(ref mut i)=>{ui.add(egui::DragValue::new(i).speed(1.0));},

        RSZValue::Int2(ref mut num2)=>{
            drag2(ui, &mut num2.x,&mut num2.y);
        },
        RSZValue::Int3(ref mut num3)=>{
            drag3(ui, &mut num3.x,&mut num3.y,&mut num3.z);
        },
        RSZValue::Int4(ref mut num4)=>{
            drag4(ui, &mut num4.x,&mut num4.y,&mut num4.z,&mut num4.w);
        },
        RSZValue::UInt2(ref mut num2)=>{
            drag2(ui, &mut num2.x,&mut num2.y);
        },
        RSZValue::UInt3(ref mut num3)=>{
            drag3(ui, &mut num3.x,&mut num3.y,&mut num3.z);
        },
        RSZValue::UInt4(ref mut num4)=>{
            drag4(ui, &mut num4.x,&mut num4.y,&mut num4.z,&mut num4.w);
        },
        //Again, mutable reference via ref keyword. 
        RSZValue::String(ref mut _string)=>{
            ui.add(egui::TextEdit::singleline( _string));
        },

        //Unk
        
        //List


        //Difficult to deal with this one right now, neither the RSZValues or TypeIDs implement default or debug.
        //For now just render the ID as a number and make a push to the parser later.
        ref _other=>{
            let error_label:String = format!("Field Type is not implemented yet! Enum type{}",field.value_type.clone() as u8);
            ui.label(error_label);
        }
    }
    //end the row and move to the next variable.
    ui.end_row();
}