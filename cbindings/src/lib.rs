extern crate rustpython_compiler;
extern crate libc;

use rustpython_compiler as compiler;
use rustpython_vm as vm;
use std::time::SystemTime;
use vm::obj::objcode::PyCodeRef;

use std::ffi::CStr;
use std::ptr;
use libc::c_char;


#[no_mangle]
pub extern fn rpy_vm_new() -> *mut vm::VirtualMachine {
    let vm = vm::VirtualMachine::new(vm::PySettings::default());
    Box::into_raw(Box::new(vm))
}

#[no_mangle]
pub unsafe extern fn rpy_new_scope_with_builtins(vm: *mut vm::VirtualMachine) -> *mut vm::scope::Scope {
    if vm.is_null() {
        return std::ptr::null_mut();
    }
    let vm = &*vm;
    let scope = vm.new_scope_with_builtins();
    Box::into_raw(Box::new(scope))
}

#[no_mangle]
pub unsafe extern fn rpy_compile_code(vm: *mut vm::VirtualMachine, code: *const c_char) -> *mut PyCodeRef {
    if vm.is_null() {
        return std::ptr::null_mut();
    }

    if code.is_null() {
        return std::ptr::null_mut();
    }

    let raw_code = CStr::from_ptr(code);

    let vm = &*vm;
    let code_obj = vm.compile(
        raw_code.to_str().unwrap(),
        compiler::compile::Mode::Single,
        "<embedded>".to_string(),
    ).unwrap();
    Box::into_raw(Box::new(code_obj))
}


#[no_mangle]
pub unsafe extern fn rpy_run_code_obj(vm: *const vm::VirtualMachine, code_obj: *const PyCodeRef, scope: *const vm::scope::Scope) {
    if vm.is_null() {
        return ;
    }

    if code_obj.is_null() {
        return ;
    }

    if scope.is_null() {
        return ;
    }
    let vm = &*vm;
    let code_obj= (&*code_obj).clone();
    let scope= (&*scope).clone();
    vm.run_code_obj(code_obj, scope).unwrap();
}


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

    let _ =  vm.run_code_obj(code_obj.clone(), scope.clone()).unwrap();
    let _ =  vm.run_code_obj(code_obj.clone(), scope.clone()).unwrap();

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("step 4, run code {}" , n.as_millis()-ts),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    return
}
