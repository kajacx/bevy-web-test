pub mod protocol_plugin {

    fn copy_slice<T>(
        _store: &mut wasmer::Store,
        _memory: &wasmer::Memory,
        _free: &wasmer::TypedFunction<(i32, i32, i32), ()>,
        _base: i32,
        _len: i32,
        _align: i32,
    ) -> Result<Vec<T>, wasmer::RuntimeError> {
        Ok(vec![])
    }

    trait Load {
        fn load<T>(&self, offset: i32) -> Result<T, wasmer::RuntimeError>;
    }

    impl Load for [u8] {
        fn load<T>(&self, _offset: i32) -> Result<T, wasmer::RuntimeError> {
            panic!()
        }
    }

    #[allow(unused_imports)]
    #[repr(C)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
    }

    pub struct Complex {
        pub name: String,
        pub colors: Vec<Color>,
        pub my: Vec<MyEnum>,
        pub mys: Vec<MyEnum>,
    }

    #[repr(u8)]
    pub enum MyEnum {
        One,
        Two,
        Three,
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
        ) -> (Self, wasmer::Instance) {
            let env = Self::add_to_imports(&mut store, imports);
            let instance = wasmer::Instance::new(&mut store, module, &*imports).unwrap();
            (Self::new(store, &instance, env).unwrap(), instance)
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
                        _ => panic!(),
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
                        _ => panic!(),
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

fn main() {
    println!("PLS WORKS");
}
