use std::{fmt::Result, time::{Duration, SystemTime, UNIX_EPOCH}};

use wasm_bindgen::prelude::*;
use js_sys::{Error, Function, Object, Reflect, WebAssembly};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Document, HtmlElement, Window};

#[wasm_bindgen]
extern "C" {
    fn alert(input: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(input: &str);
}

#[wasm_bindgen]
pub fn add(left: u32, right: u32){
    let result = left + right;
    log(&result.to_string());
    // alert(&result.to_string());
}

macro_rules! console_logger {
    ($($val:tt)*) => {
        log(format_args!($($val)*).to_string().as_str())
    };
}

const WASM: &[u8] = include_bytes!("first_contract.wasm");

async fn runner()-> Result{
    let a = JsFuture::from(WebAssembly::instantiate_buffer(&WASM, &Object::new())).await.unwrap();
    let b = Reflect::get(&a, &"insstance".into()).unwrap().dyn_into::<WebAssembly::Instance>().unwrap();

    let c = b.exports();

    console_logger!("three: {:?}", c);

    Ok(())
}

#[wasm_bindgen(module = "/defined_in_js.js")]
extern "C" {
    fn name()-> String;
    
    type MyClass;

    #[wasm_bindgen(constructor)]
    fn new()-> MyClass; 

    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass)->u32;

    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, arg: u32);

    #[wasm_bindgen(method)]
    fn render(this: &MyClass)->String;
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter{
    count: i32,
    key: char
}

#[wasm_bindgen]
impl Counter {
    pub fn new (key: char, count: i32)->Self{
        Counter{
            count,
            key
        }
    }
    pub fn key (&self)-> char{
        self.key
    }
    pub fn count (&self)-> i32{
        self.count
    }
    pub fn increment(&mut self){
        self.count += 1;
    }
    pub fn updatekey(&mut self, key: char){
        self.key = key;
    }
}


#[wasm_bindgen(start)]
pub fn Start(){
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();


    let performance = window.performance().expect("Performance");

    console_logger!("Current performance: {}", performance.now());


    setup_clock(&window, &document);

    let p = document.create_element("p").unwrap();
    p.set_text_content(Some("Hello from Rust!"));
    
    body.append_child(&p).unwrap();

    let start = perf_to_sys(performance.timing().request_start());
    let stop = perf_to_sys(performance.timing().response_end());

    console_logger!("Started at {}", humantime::format_rfc3339(start));
    console_logger!("Stoped at {}", humantime::format_rfc3339(stop));

}

fn perf_to_sys(amt: f64)-> SystemTime{
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;

    UNIX_EPOCH + Duration::new(secs, nanos)
}

fn setup_clock(window: &Window, document: &Document){
    let mut count = 0;
    let func = Closure::<dyn Fn()>::new({
        let doc = document.clone();
        move|| {
            let binding = doc.clone().get_element_by_id("loading").unwrap();
            let val = binding.dyn_ref::<HtmlElement>().expect("ele");

            val.set_hidden(!val.hidden());
            console_logger!("Hello from Rust! {}", count.clone());
        }}
    );
    let res = window.set_interval_with_callback_and_timeout_and_arguments_0(
        func.as_ref().unchecked_ref(), 1000
    ).unwrap();
    let button = document.query_selector("button").unwrap();
    func.forget();
}

#[wasm_bindgen]
pub fn call_run(){
    
    spawn_local(async{
        runner().await.unwrap_throw()
    });
}

#[wasm_bindgen]
pub fn run(){
    log("Works");
    let x = MyClass::new();

    let number = x.number();
    log(&number.to_string());
    x.set_number(10);
    let number2 = x.number();
    log(&number2.to_string());
    log(&name())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
