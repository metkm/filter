#![no_std]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[cfg(not(test))]
extern crate wdk_panic;


#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

mod kernel;

use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT};
use kernel::{WdfDriverInitNonPnpDriver, WDF_DRIVER_CONFIG, WDF_DRIVER_CONFIG_INIT};
use wdk::println;

unsafe extern "C" fn filter_unload(_driver: *mut DRIVER_OBJECT) {
    println!("unloading driver");
}

unsafe extern "C" fn driver_config_unload() {
    println!("unloading driver config");
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

    let mut config = WDF_DRIVER_CONFIG {
        Size: 0,
        DriverInitFlags: WdfDriverInitNonPnpDriver,
        DriverPoolTag: 0,
        EvtDriverDeviceAdd: None,
        EvtDriverUnload: Some(driver_config_unload)
    };
    config.Size = size_of_val(&config) as u32;

    unsafe {
        WDF_DRIVER_CONFIG_INIT(
            config,
            None
        );
    }

    0
}
