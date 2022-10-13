use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct Canvas {
    width: f64,
    height: f64,
}
#[wasm_bindgen(start)]
pub fn start() {
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

    let canv = Canvas {
        width: canvas.width() as f64,
        height: canvas.height() as f64,
    };
    
    context.begin_path();

    context
        .arc(canv.width/2.0, canv.height/2.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}

#[wasm_bindgen]
pub fn update() {
    let document = web_sys::window().unwrap().document().unwrap();
    let get = document.get_element_by_id("radius")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();
            
    let radius = get.parse::<f64>().unwrap();
    draw(radius);
    }
    



pub fn draw(radius: f64) {
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

    
    let canv = Canvas {
        width: canvas.width() as f64,
        height: canvas.height() as f64,
    };
    context.clear_rect(0.0, 0.0, canv.width, canv.height); 
    context.begin_path();

    context
        .arc(canv.width/2.0, canv.height/2.0, radius, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}