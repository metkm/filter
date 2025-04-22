#![no_std]

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, ntddk::IoDeleteDevice};

unsafe extern "C" fn boost_unload(driver: *mut DRIVER_OBJECT) {
    unsafe {
        IoDeleteDevice((*driver).DeviceObject);
    }
}

#[unsafe(export_name = "DriverEntry")] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: PDRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    wdk::println!("driver entry");

    unsafe {
        (*driver).DriverUnload = Some(boost_unload);
    }

    0
}
