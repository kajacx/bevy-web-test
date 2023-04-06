mod utils;

use utils::set_panic_hook;
use wai_bindgen_wasmer::wasmer::*;
use wasm_bindgen::prelude::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/bevy_plugin.wasm");

wai_bindgen_wasmer::import!("../protocol-plugin.wai");

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-only!");
}

#[wasm_bindgen]
pub fn print_color() {
    set_panic_hook();
    alert(&format!("COLOR IS: {:?}", get_color()));
}

fn get_color() -> protocol_plugin::Color {
    //let mut store = Store::new(Engine::default());
    let mut store = Store::new();

    let module = Module::new(&store, PLUGIN_BYTES).expect("should create module");

    let (plugin, _) =
        protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut imports! {})
            .expect("should create instance");

    plugin.get_color(&mut store).expect("should get color")
}
