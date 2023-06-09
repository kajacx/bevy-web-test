use wasm_bindgen::prelude::*;
use wasmer::*;

struct Env {
    instance: Option<&'static Instance>,
    store: usize,
}

unsafe impl Send for Env {}
unsafe impl Sync for Env {}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn main() -> Result<u32, Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer_plugin.wasm"
    )
    .as_ref();

    // Create the store
    // let mut store = Store::new();
    let mut store = Store::new(Engine::default());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    let env = FunctionEnv::new(
        &mut store,
        Env {
            instance: None,
            store: 0,
        },
    );

    // Create an empty import object.
    let import_object = imports! {
        "my_imports" => {
            "transform_string" => Function::new_typed_with_env(&mut store, &env, |envf: FunctionEnvMut<Env>, input: u64| {
                let store =unsafe {
                    &mut * (envf.data().store as *mut Store)
                };
                let instance = envf.data().instance.unwrap();
                let memory = instance.exports.get_memory("memory").unwrap();

                let bytes = import_from_plugin(instance, memory, store, input);
                let text = String::from_utf8(bytes).unwrap();

                let transmuted = transmute_string(text);

                let exported = export_to_plugin(&memory,  store, instance, &transmuted.into_bytes());
                exported
            })
        }
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let memory = instance.exports.get_memory("memory").unwrap();

    let store_address = &store as *const _ as usize;
    let mut env_mut = env.as_mut(&mut store);
    env_mut.store = store_address;
    unsafe {
        env_mut.instance = Some(&*(&instance as *const _));
    }

    let mut compare_string = String::new();

    for i in 0..1_000_000u32 {
        if i % 10_000 == 0 {
            println!("{}: {}", i, compare_string.len());
        }

        if compare_string.len() < 1000 {
            grow_strings(memory, &mut store, &instance, &mut compare_string, i);
            continue;
        }
        if compare_string.len() > 1000000 {
            shrink_strings(&mut store, &instance, &mut compare_string, i % 100);
            continue;
        }
        if i % 13 < 5 {
            grow_strings(memory, &mut store, &instance, &mut compare_string, i);
        } else {
            shrink_strings(&mut store, &instance, &mut compare_string, i % 100);
        }
    }

    Ok(compare_string.len() as u32)
}

#[wasm_bindgen]
pub fn run_main() -> u32 {
    console_error_panic_hook::set_once();
    main().expect("should run main")
}

#[wasm_bindgen]
pub fn call_take_u32() {
    call_take_u32_raw()
}

pub fn call_take_u32_raw() {
    console_error_panic_hook::set_once();

    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer_plugin.wasm"
    )
    .as_ref();

    let mut store = Store::new(Engine::default());
    let module = Module::new(&store, wasm_bytes).unwrap();

    let import_object = imports! {
           "my_imports" => {
    "transform_string" => Function::new_typed(&mut store, | input: u64| {
               })
           }
       };
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let text = "".to_string();

    let add_ten = instance
        .exports
        .get_typed_function::<u32, u32>(&store, "add_ten")
        .unwrap();
    let ten1 = add_ten.call(&mut store, 50).unwrap();
    let ten2 = add_ten.call(&mut store, u32::MAX / 2 - 2).unwrap();
    let ten3 = add_ten.call(&mut store, u32::MAX - 6).unwrap();
    let ten4 = add_ten.call(&mut store, u32::MAX / 2 + 500).unwrap();

    let text = text + &format!("Results: {ten1}, {ten2}, {ten3}, {ten4}\n");

    let sub_ten = instance
        .exports
        .get_typed_function::<u32, u32>(&store, "sub_ten")
        .unwrap();
    let ten1 = sub_ten.call(&mut store, 50).unwrap();
    let ten2 = sub_ten.call(&mut store, u32::MAX / 2 + 2).unwrap();
    let ten3 = sub_ten.call(&mut store, 6).unwrap();
    let ten4 = sub_ten.call(&mut store, u32::MAX / 2 + 500).unwrap();

    let text = text + &format!("Results: {ten1}, {ten2}, {ten3}, {ten4}\n");

    alert("XA");
    let add_twenty = instance
        .exports
        .get_typed_function::<i64, i64>(&store, "add_twenty")
        .unwrap();
    alert("XA1");
    let ten1 = add_twenty.call(&mut store, 50).unwrap();
    alert("XA2");
    let ten2 = add_twenty.call(&mut store, -2).unwrap_or(666);
    alert("XA3");
    let ten3 = add_twenty.call(&mut store, i64::MAX - 6).unwrap_or(666);
    alert("XA4");
    let ten4 = add_twenty.call(&mut store, i64::MIN + 500).unwrap_or(666);
    alert("XA5");

    let text = text + &format!("Results: {ten1}, {ten2}, {ten3}, {ten4}\n");

    alert("A");
    let add_twenty = instance
        .exports
        .get_typed_function::<u64, u64>(&store, "add_twenty")
        .unwrap();
    alert("A1");
    let ten1 = add_twenty.call(&mut store, 50).unwrap();
    alert("A2");
    let ten2 = add_twenty.call(&mut store, u64::MAX / 2 - 2).unwrap_or(666);
    alert("A3");
    let ten3 = add_twenty.call(&mut store, u64::MAX - 6).unwrap_or(666);
    alert("A4");
    let ten4 = add_twenty
        .call(&mut store, u64::MAX / 2 + 500)
        .unwrap_or(666);
    alert("A5");

    let text = text + &format!("Results: {ten1}, {ten2}, {ten3}, {ten4}\n");

    alert("B");
    let sub_twenty = instance
        .exports
        .get_typed_function::<u64, u64>(&store, "sub_twenty")
        .unwrap();
    alert("B1");
    let ten1 = sub_twenty.call(&mut store, 50).unwrap();
    alert("B2");
    let ten2 = sub_twenty.call(&mut store, u64::MAX / 2 + 2).unwrap();
    alert("B3");
    let ten3 = sub_twenty.call(&mut store, 6).unwrap_or(666);
    alert("B4");
    let ten4 = sub_twenty.call(&mut store, u64::MAX / 2 + 500).unwrap();
    alert("B5");

    let text = text + &format!("Results: {ten1}, {ten2}, {ten3}, {ten4}\n");

    alert(&text);
}

fn grow_strings(
    memory: &Memory,
    store: &mut Store,
    instance: &Instance,
    compare_string: &mut String,
    n: u32,
) {
    let appendings = format!("Growing: {n}, ");
    append_string(compare_string, appendings.clone(), transmute_string);

    let exported = export_to_plugin(memory, store, instance, appendings.as_bytes());

    let push_str = instance
        .exports
        .get_typed_function::<i64, i64>(&store, "push_string")
        .unwrap();
    let ret = push_str.call(store, exported as i64).unwrap() as u64;

    let imported = import_from_plugin(instance, memory, store, ret);
    let check = String::from_utf8(imported).unwrap();

    let a: &str = compare_string.as_str();
    let b: &str = check.as_str();
    assert_eq!(a, b);
}

fn shrink_strings(store: &mut Store, instance: &Instance, compare_string: &mut String, n: u32) {
    shrink_string(compare_string, n);
    let rm_chars = instance
        .exports
        .get_typed_function::<i32, ()>(&store, "remove_chars")
        .unwrap();
    rm_chars.call(store, n as i32).unwrap();
}

fn import_from_plugin(
    instace: &Instance,
    memory: &Memory,
    store: &mut Store,
    fatptr: u64,
) -> Vec<u8> {
    let (addr, len) = from_fatptr(fatptr);
    let mut bytes = vec![0; len];
    let view = memory.view(store);
    view.read(addr as u64, &mut bytes[0..len]).unwrap();

    let free = instace
        .exports
        .get_typed_function::<i64, ()>(store, "free_from_host")
        .unwrap();
    free.call(store, fatptr as i64).unwrap();

    bytes
}

fn export_to_plugin(memory: &Memory, store: &mut Store, instance: &Instance, data: &[u8]) -> u64 {
    let allocate = instance
        .exports
        .get_typed_function::<i32, i64>(&store, "allocate_for_host")
        .unwrap();
    let mut allocate = |size: u32| allocate.call(store, size as i32).unwrap() as u64;

    let fatptr = allocate(data.len() as u32);
    let (addr, _) = from_fatptr(fatptr);
    let view = memory.view(store);
    view.write(addr as u64, data).unwrap();
    fatptr
}

fn transmute_string(text: String) -> String {
    text + " transmuted in host"
}

// Common between host and plugin:

fn append_string(
    changed_string: &mut String,
    appended_string: String,
    mut transmutor: impl FnMut(String) -> String,
) {
    let appended_string = appended_string + " appended";
    let appended_string = transmutor(appended_string);
    changed_string.push_str(&appended_string);
}

fn shrink_string(changed_string: &mut String, byte_count: u32) {
    let len = changed_string.len();
    changed_string.replace_range((len - byte_count as usize)..len, "");
}

fn from_fatptr(fatptr: u64) -> (usize, usize) {
    let addr = fatptr as u32 as usize;
    let len = (fatptr >> 32) as usize;
    (addr, len)
}
