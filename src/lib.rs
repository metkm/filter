#![no_std]

#[cfg(not(test))]
extern crate wdk_panic;

use wdk::println;
#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT};

unsafe extern "C" fn filter_unload(_driver: *mut DRIVER_OBJECT) {
    println!("unloading driver");
}

#[unsafe(export_name = "DriverEntry")] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: PDRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    wdk::println!("driver entry");

    unsafe {
        (*driver).DriverUnload = Some(filter_unload);
    }

    0
}
