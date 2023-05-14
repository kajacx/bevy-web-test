pub mod protocol_plugin {
    #[allow(unused_imports)]
    use wai_bindgen_wasmer::{anyhow, wasmer};
    #[repr(C)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Color {}
    #[automatically_derived]
    impl ::core::clone::Clone for Color {
        #[inline]
        fn clone(&self) -> Color {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    impl core::fmt::Debug for Color {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Color")
                .field("r", &self.r)
                .field("g", &self.g)
                .field("b", &self.b)
                .finish()
        }
    }
    impl wai_bindgen_wasmer::Endian for Color {
        fn into_le(self) -> Self {
            Self {
                r: self.r.into_le(),
                g: self.g.into_le(),
                b: self.b.into_le(),
            }
        }
        fn from_le(self) -> Self {
            Self {
                r: self.r.from_le(),
                g: self.g.from_le(),
                b: self.b.from_le(),
            }
        }
    }
    unsafe impl wai_bindgen_wasmer::AllBytesValid for Color {}
    pub struct Complex {
        pub name: String,
        pub colors: Vec<Color>,
        pub my: Vec<MyEnum>,
        pub mys: Vec<MyEnum>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Complex {
        #[inline]
        fn clone(&self) -> Complex {
            Complex {
                name: ::core::clone::Clone::clone(&self.name),
                colors: ::core::clone::Clone::clone(&self.colors),
                my: ::core::clone::Clone::clone(&self.my),
                mys: ::core::clone::Clone::clone(&self.mys),
            }
        }
    }
    impl core::fmt::Debug for Complex {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Complex")
                .field("name", &self.name)
                .field("colors", &self.colors)
                .field("my", &self.my)
                .field("mys", &self.mys)
                .finish()
        }
    }
    #[repr(u8)]
    pub enum MyEnum {
        One,
        Two,
        Three,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MyEnum {
        #[inline]
        fn clone(&self) -> MyEnum {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MyEnum {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MyEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MyEnum {
        #[inline]
        fn eq(&self, other: &MyEnum) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for MyEnum {}
    #[automatically_derived]
    impl ::core::cmp::Eq for MyEnum {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl core::fmt::Debug for MyEnum {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                MyEnum::One => f.debug_tuple("MyEnum::One").finish(),
                MyEnum::Two => f.debug_tuple("MyEnum::Two").finish(),
                MyEnum::Three => f.debug_tuple("MyEnum::Three").finish(),
            }
        }
    }
    /// Auxiliary data associated with the wasm exports.
    pub struct ProtocolPluginData {}
    #[automatically_derived]
    impl ::core::default::Default for ProtocolPluginData {
        #[inline]
        fn default() -> ProtocolPluginData {
            ProtocolPluginData {}
        }
    }
    pub struct ProtocolPlugin {
        #[allow(dead_code)]
        env: wasmer::FunctionEnv<ProtocolPluginData>,
        func_add_three: wasmer::TypedFunction<i32, i32>,
        func_canonical_abi_free: wasmer::TypedFunction<(i32, i32, i32), ()>,
        func_get_color: wasmer::TypedFunction<(), i32>,
        func_get_complex: wasmer::TypedFunction<(), i32>,
        memory: wasmer::Memory,
    }
    impl ProtocolPlugin {
        #[allow(unused_variables)]
        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `ImportObject` provided.
        ///
        /// This function returns the `ProtocolPluginData` which needs to be
        /// passed through to `ProtocolPlugin::new`.
        fn add_to_imports(
            mut store: impl wasmer::AsStoreMut,
            imports: &mut wasmer::Imports,
        ) -> wasmer::FunctionEnv<ProtocolPluginData> {
            let env = wasmer::FunctionEnv::new(&mut store, ProtocolPluginData::default());
            env
        }
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        ///
        /// The `imports` provided will have intrinsics added to it
        /// automatically, so it's not necessary to call
        /// `add_to_imports` beforehand. This function will
        /// instantiate the `module` otherwise using `imports`, and
        /// both an instance of this structure and the underlying
        /// `wasmer::Instance` will be returned.
        pub fn instantiate(
            mut store: impl wasmer::AsStoreMut,
            module: &wasmer::Module,
            imports: &mut wasmer::Imports,
        ) -> anyhow::Result<(Self, wasmer::Instance)> {
            let env = Self::add_to_imports(&mut store, imports);
            let instance = wasmer::Instance::new(&mut store, module, &*imports)?;
            Ok((Self::new(store, &instance, env)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// and wrap them all up in the returned structure which can
        /// be used to interact with the wasm module.
        pub fn new(
            store: impl wasmer::AsStoreMut,
            _instance: &wasmer::Instance,
            env: wasmer::FunctionEnv<ProtocolPluginData>,
        ) -> Result<Self, wasmer::ExportError> {
            let func_add_three = _instance.exports.get_typed_function(&store, "add-three")?;
            let func_canonical_abi_free = _instance
                .exports
                .get_typed_function(&store, "canonical_abi_free")?;
            let func_get_color = _instance.exports.get_typed_function(&store, "get-color")?;
            let func_get_complex = _instance
                .exports
                .get_typed_function(&store, "get-complex")?;
            let memory = _instance.exports.get_memory("memory")?.clone();
            Ok(ProtocolPlugin {
                func_add_three,
                func_canonical_abi_free,
                func_get_color,
                func_get_complex,
                memory,
                env,
            })
        }
        pub fn add_three(
            &self,
            store: &mut wasmer::Store,
            number: u32,
        ) -> Result<u32, wasmer::RuntimeError> {
            let result0 = self
                .func_add_three
                .call(store, wai_bindgen_wasmer::rt::as_i32(number))?;
            Ok(result0 as u32)
        }
        pub fn get_color(&self, store: &mut wasmer::Store) -> Result<Color, wasmer::RuntimeError> {
            let _memory = &self.memory;
            let result0 = self.func_get_color.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<f32>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<f32>(result0 + 4)?;
            let _memory_view = _memory.view(&store);
            let load3 = unsafe { _memory_view.data_unchecked_mut() }.load::<f32>(result0 + 8)?;
            Ok(Color {
                r: load1,
                g: load2,
                b: load3,
            })
        }
        pub fn get_complex(
            &self,
            store: &mut wasmer::Store,
        ) -> Result<Complex, wasmer::RuntimeError> {
            let func_canonical_abi_free = &self.func_canonical_abi_free;
            let _memory = &self.memory;
            let result0 = self.func_get_complex.call(store)?;
            let _memory_view = _memory.view(&store);
            let load1 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 0)?;
            let _memory_view = _memory.view(&store);
            let load2 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 4)?;
            let ptr3 = load1;
            let len3 = load2;
            let data3 = copy_slice(store, _memory, func_canonical_abi_free, ptr3, len3, 1)?;
            let _memory_view = _memory.view(&store);
            let load4 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 8)?;
            let _memory_view = _memory.view(&store);
            let load5 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 12)?;
            let ptr6 = load4;
            let len6 = load5;
            let _memory_view = _memory.view(&store);
            let load7 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 16)?;
            let _memory_view = _memory.view(&store);
            let load8 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 20)?;
            let len10 = load8;
            let base10 = load7;
            let mut result10 = Vec::with_capacity(len10 as usize);
            for i in 0..len10 {
                let base = base10 + i * 1;
                result10.push({
                    let _memory_view = _memory.view(&store);
                    let load9 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(base + 0)?;
                    match i32::from(load9) {
                        0 => MyEnum::One,
                        1 => MyEnum::Two,
                        2 => MyEnum::Three,
                        _ => return Err(invalid_variant("MyEnum")),
                    }
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base10, len10 * 1, 1)?;
            let _memory_view = _memory.view(&store);
            let load11 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 24)?;
            let _memory_view = _memory.view(&store);
            let load12 = unsafe { _memory_view.data_unchecked_mut() }.load::<i32>(result0 + 28)?;
            let len14 = load12;
            let base14 = load11;
            let mut result14 = Vec::with_capacity(len14 as usize);
            for i in 0..len14 {
                let base = base14 + i * 1;
                result14.push({
                    let _memory_view = _memory.view(&store);
                    let load13 =
                        unsafe { _memory_view.data_unchecked_mut() }.load::<u8>(base + 0)?;
                    match i32::from(load13) {
                        0 => MyEnum::One,
                        1 => MyEnum::Two,
                        2 => MyEnum::Three,
                        _ => return Err(invalid_variant("MyEnum")),
                    }
                });
            }
            func_canonical_abi_free.call(&mut store.as_store_mut(), base14, len14 * 1, 1)?;
            Ok(Complex {
                name: String::from_utf8(data3)
                    .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?,
                colors: copy_slice(store, _memory, func_canonical_abi_free, ptr6, len6, 4)?,
                my: result10,
                mys: result14,
            })
        }
    }
    use wai_bindgen_wasmer::rt::copy_slice;
    use wai_bindgen_wasmer::rt::invalid_variant;
    use wai_bindgen_wasmer::rt::RawMem;
    #[allow(unused_imports)]
    use wasmer::AsStoreMut as _;
    #[allow(unused_imports)]
    use wasmer::AsStoreRef as _;
}
const _: &str = "add-three: func(number: u32) -> u32\n\nget-color: func() -> color\n\nget-complex: func() -> complex\n\nrecord color {\n  r: float32,\n  g: float32,\n  b: float32,\n}\n\nrecord complex {\n  name: string,\n  colors: list<color>,\n  my: list<my-enum>,\n  mys: list<my-enum>,\n}\n\nenum my-enum {\n  one,\n  two,\n  three,\n}\n";
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[automatically_derived]
const _: () = {
    #[no_mangle]
    #[doc(hidden)]
    pub extern "C" fn __wbindgen_describe___wbg_alert_5a03d83ccdc0bd9f() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
};
#[allow(nonstandard_style)]
#[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
///
fn alert(s: &str) {
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[link(wasm_import_module = "__wbindgen_placeholder__")]
    extern "C" {
        fn __wbg_alert_5a03d83ccdc0bd9f(s: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi)
            -> ();
    }
    unsafe {
        let _ret = {
            let s = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
            __wbg_alert_5a03d83ccdc0bd9f(s)
        };
        ()
    }
}
#[cfg(target_arch = "wasm32")]
#[automatically_derived]
const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 139usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.84\",\"version\":\"0.2.84\"}U\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1c__wbg_alert_5a03d83ccdc0bd9f\x00\x00\x00\x00\x01\x01\x01s\x00\x05alert\x01\x00\x00\x00\x00\x00\x1awasm-only-01cd770bba33d2a1\x00\x00";
};
#[allow(dead_code)]
pub fn greet() {
    alert("Hello, wasm-only!");
}
#[automatically_derived]
const _: () = {
    #[export_name = "greet"]
    pub unsafe extern "C" fn __wasm_bindgen_generated_greet(
    ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let _ret = greet();
            _ret
        };
        <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[automatically_derived]
const _: () = {
    #[no_mangle]
    #[doc(hidden)]
    pub extern "C" fn __wbindgen_describe_greet() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
};
#[cfg(target_arch = "wasm32")]
#[automatically_derived]
const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 107usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.84\",\"version\":\"0.2.84\"}5\x00\x00\x00\x01\x00\x00\x00\x00\x00\x05greet\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1awasm-only-01cd770bba33d2a1\x00\x00";
};
#[allow(dead_code)]
pub fn print_eight() {
    set_panic_hook();
    alert(&{
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["5 + 3 = "],
            &[::core::fmt::ArgumentV1::new_debug(&add_three(5))],
        ));
        res
    });
}
#[automatically_derived]
const _: () = {
    #[export_name = "print_eight"]
    pub unsafe extern "C" fn __wasm_bindgen_generated_print_eight(
    ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let _ret = print_eight();
            _ret
        };
        <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[automatically_derived]
const _: () = {
    #[no_mangle]
    #[doc(hidden)]
    pub extern "C" fn __wbindgen_describe_print_eight() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
};
#[cfg(target_arch = "wasm32")]
#[automatically_derived]
const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 113usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.84\",\"version\":\"0.2.84\"};\x00\x00\x00\x01\x00\x00\x00\x00\x00\x0bprint_eight\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1awasm-only-01cd770bba33d2a1\x00\x00";
};
#[allow(dead_code)]
pub fn print_color() {
    set_panic_hook();
    alert(&{
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["COLOR IS: "],
            &[::core::fmt::ArgumentV1::new_debug(&get_color())],
        ));
        res
    });
}
#[automatically_derived]
const _: () = {
    #[export_name = "print_color"]
    pub unsafe extern "C" fn __wasm_bindgen_generated_print_color(
    ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let _ret = print_color();
            _ret
        };
        <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[automatically_derived]
const _: () = {
    #[no_mangle]
    #[doc(hidden)]
    pub extern "C" fn __wbindgen_describe_print_color() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
};
#[cfg(target_arch = "wasm32")]
#[automatically_derived]
const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 113usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.84\",\"version\":\"0.2.84\"};\x00\x00\x00\x01\x00\x00\x00\x00\x00\x0bprint_color\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1awasm-only-01cd770bba33d2a1\x00\x00";
};
fn get_color() -> protocol_plugin::Color {
    let (mut store, module) = create_store();
    let (plugin, _) = protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut {
        #[allow(unused_mut)]
        let mut import_object = ::wasmer::Imports::new();
        import_object
    })
    .expect("should create instance");
    plugin.get_color(&mut store).expect("should get color")
}
fn add_three(number: u32) -> u32 {
    let (mut store, module) = create_store();
    let (plugin, _) = protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut {
        #[allow(unused_mut)]
        let mut import_object = ::wasmer::Imports::new();
        import_object
    })
    .expect("should create instance");
    plugin
        .add_three(&mut store, number)
        .expect("should add three")
}
fn create_store() -> (Store, Module) {
    let store = Store::new(Engine::default());
    let bytes = PLUGIN_BYTES;
    let module = Module::new(&store, &bytes).expect("should create module");
    (store, module)
}
fn main() {
    println!("PLS WORKS");
}
