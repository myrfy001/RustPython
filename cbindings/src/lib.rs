extern crate rustpython_compiler;

use rustpython_compiler as compiler;
use rustpython_vm as vm;
use std::time::SystemTime;


#[no_mangle]
pub extern fn test_main() {

    let mut ts:u128 = 0;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => ts=n.as_millis(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    let vm = vm::VirtualMachine::new(vm::PySettings::default());

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("step 1, create new vm {}" , n.as_millis()-ts),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    let scope = vm.new_scope_with_builtins();

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("step 2, make new scope {}" , n.as_millis()-ts),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    let code_obj = vm
        .compile(
            r#"print("Hello World!")"#,
            compiler::compile::Mode::Exec,
            "<embedded>".to_string(),
        )
        .map_err(|err| vm.new_syntax_error(&err)).unwrap();

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("step 3, compile code {}" , n.as_millis()-ts),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    let _ =  vm.run_code_obj(code_obj, scope).unwrap();

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("step 4, run code {}" , n.as_millis()-ts),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    return
}
