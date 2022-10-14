use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[wasm_bindgen(start)]
pub fn start() {
    draw(400.0, 1000.0, 150.0, 0.0);
}
    


pub fn draw(startX: f64, startY: f64, len: f64, angle: f64) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    context.begin_path();
    context.save();
    // all canvas function here
    
    context
        .translate(startX, startY)
        .unwrap();
    context
        .rotate(angle * std::f64::consts::PI/180.0 )
        .unwrap();
    context.move_to(0.0, 0.0);
    context.line_to(0.0, -len);
    //
    context.stroke();
    if len < 10.0 {
        context.restore();
        return;
    }
    draw(0.0, -len, len*0.8, -15.0);
    draw(0.0, -len, len*0.8, 15.0);

    context.restore();
}