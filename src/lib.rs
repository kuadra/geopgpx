use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_json(a: JsValue);
}

#[wasm_bindgen]
pub fn write_to_console(name: &str) {
    log_str(name);
}

#[wasm_bindgen]
pub fn write_json_to_console(name: JsValue) {
    log_json(name);
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


#[derive(Serialize, Deserialize, Debug)]
pub struct GpxData {
    x : f64,
    y : f64
}

// #[wasm_bindgen]
// pub fn read_gpx_json(i : JsValue) {
//     write_json_to_console(i);
//     crunch_data(i);
// }

#[wasm_bindgen]
pub fn read_gpx_json(val: JsValue) -> Result<(), JsValue> {
    let example: GpxData = serde_wasm_bindgen::from_value(val)?;
    /* …do something with `example`… */
    write_to_console(format!("{:?}", example).as_str());
    let p =crunch_data(example);
    write_to_console(format!("{:?}", p).as_str());
    Ok(())
}

fn crunch_data(input : GpxData) -> GpxData{
    let o = GpxData{x : input.x+input.x, y : input.y};
    return o;
}

#[wasm_bindgen]
pub fn test_log(i : &str) {
    write_to_console(i)
}