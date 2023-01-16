use gpx::errors::GpxError;
use gpx::read;
use gpx::Gpx;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_js_value(a: JsValue);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_js_vec(a: Vec<JsValue>);
}

#[wasm_bindgen]
pub fn print_string(name: &str) {
    log_str(name);
}

pub fn print_js_value(name: JsValue) {
    log_js_value(name);
}

pub fn print_js_vec(name: Vec<JsValue>) {
    log_js_vec(name);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GpxData {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub fn read_gpx_json(val: JsValue) -> Result<(), JsValue> {
    let example: GpxData = serde_wasm_bindgen::from_value(val)?;
    /* …do something with `example`… */
    print_string(format!("{:?}", example).as_str());
    let p = crunch_data(example);
    print_string(format!("{:?}", p).as_str());
    Ok(())
}

fn crunch_data(input: GpxData) -> GpxData {
    let o = GpxData {
        x: input.x + 5.0,
        y: input.y,
    };
    return o;
}

#[wasm_bindgen]
pub fn work(val: JsValue) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let example: GpxData = serde_wasm_bindgen::from_value(val)?;
    print_string(format!("input: {:?}", example).as_str());
    let p = crunch_data(example);
    let p_json = serde_wasm_bindgen::to_value(&p);
    print_string(format!("{:?}", p).as_str());
    p_json
}

#[wasm_bindgen]
pub fn work2(val: String) -> Vec<JsValue> {
    let data = BufReader::new(val.as_bytes());
    let gpx: Result<Gpx, GpxError> = read(data);

    let mut out = Vec::<JsValue>::new();
    for p in &gpx.unwrap().tracks[0].segments[0].points {
        let k = serde_wasm_bindgen::to_value(&p.point());
        out.push(k.unwrap());
    }
    return out;
}

// pub fn foo() {
//     let file;
//     // This XML file actually exists — try it for yourself!
//     match File::open("res/2023-01-02_1004518380_Frascati-Pescasseroli 3.gpx") {
//         Ok(o) =>file = o,
//         Err(e) => panic!("Cant open file: {}", e)
//     }
//     let reader = BufReader::new(file);

//     // read takes any io::Read and gives a Result<Gpx, Error>.
//     let gpx: Gpx = read(reader).unwrap();

//     //let mut file = File::create("foo.txt").unwrap();

//     for p in &gpx.tracks[0].segments[0].points {
//         let j = serde_json::to_string(&p.point()).unwrap();
//     }
// }