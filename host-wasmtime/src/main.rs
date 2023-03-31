use bevy::prelude::*;
use wai_bindgen_wasmer::wasmer::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/wai_sample_plugin.wasm");

wai_bindgen_wasmer::import!("../protocol-plugin.wai");

fn main() {
    let plugin_color = get_color();
    let bevy_color = Color::rgb(plugin_color.r, plugin_color.g, plugin_color.b);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(bevy_color))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_color() -> protocol_plugin::Color {
    let mut store = Store::new(Engine::default());

    let module = Module::new(&store, PLUGIN_BYTES).expect("should create module");

    let (plugin, _) =
        protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut imports! {})
            .expect("should create instance");

    plugin.get_color(&mut store).expect("should get color")
}
