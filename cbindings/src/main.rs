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
    println!("111");
    let vm = &*vm;
    println!("222");
    let code_obj= std::ptr::read(code_obj);
    println!("333");
    let scope= std::ptr::read(scope);
    println!("444");
    let tt1 = code_obj;
    println!("eeeee");
    let tt2 = scope.clone();
    println!("rrrrr");
    vm.run_code_obj(tt1, tt2).unwrap();
    println!("555");
}


fn main() {
    let vm = vm::VirtualMachine::new(vm::PySettings::default());
    let vm_ptr = Box::into_raw(Box::new(vm));
    
    let vm = unsafe{&*vm_ptr};
    let scope = vm.new_scope_with_builtins();
    let scope_ptr = Box::into_raw(Box::new(scope));

    let code_obj = vm.compile(
        "print(\"777788877788\")",
        compiler::compile::Mode::Single,
        "<embedded>".to_string(),
    ).unwrap();
    let code_ptr = Box::into_raw(Box::new(code_obj));

    let vm1 = unsafe{&*vm};
    let code_obj_1 = unsafe{(&*code_ptr).clone()};
    let scope_1= unsafe{(&*scope_ptr).clone()};
    vm1.run_code_obj(code_obj_1, scope_1).unwrap();

    let vm2 = unsafe{&*vm};
    let code_obj_2 = unsafe{(&*code_ptr).clone()};
    let scope_2= unsafe{(&*scope_ptr).clone()};
    vm2.run_code_obj(code_obj_2, scope_2).unwrap();

    return
}
