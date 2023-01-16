use gpx::errors::GpxError;
use gpx::read;
use gpx::Gpx;
use std::io::BufReader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn print_string(name: &str) {
    log_str(name);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_js_value(a: JsValue);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_js_vec(a: Vec<JsValue>);
}

pub fn print_js_value(name: JsValue) {
    log_js_value(name);
}

pub fn print_js_vec(name: Vec<JsValue>) {
    log_js_vec(name);
}

#[wasm_bindgen]
pub fn get_points_from_gpx(val: String) -> Vec<JsValue> {
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