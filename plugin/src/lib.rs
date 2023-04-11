wai_bindgen_rust::export!("../protocol-plugin.wai");

struct ProtocolPlugin;

impl protocol_plugin::ProtocolPlugin for ProtocolPlugin {
    fn add_three(number: u32) -> u32 {
        number + 3
    }

    fn get_color() -> protocol_plugin::Color {
        protocol_plugin::Color {
            r: 0.2,
            g: 1.0,
            b: 0.4,
        }
    }
}
