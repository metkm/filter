use wdk_sys::{NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT};

pub const WdfDriverInitNonPnpDriver: u32 = 0x00000001;
pub const WdfDriverInitNoDispatchOverride: u32 = 0x00000002;
pub const WdfVerifyOn: u32 = 0x00000004;
pub const WdfVerifierOn: u32 = 0x00000008;

#[repr(C)]
pub struct WDF_OBJECT_ATTRIBUTES {
    Size: u32,
    EvtCleanupCallback: unsafe extern "C" fn(),
    EvtDestroyCallback: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PWDF_DRIVER_CONFIG {
    pub Size: u32,
    pub EvtDriverDeviceAdd: Option<unsafe extern "C" fn()>,
    pub EvtDriverUnload: Option<unsafe extern "C" fn()>,
    pub DriverInitFlags: u32,
    pub DriverPoolTag: u32,
}

type PFN_WDF_DRIVER_DEVICE_ADD = Option<unsafe extern "C" fn()>;

unsafe extern "C" {
    pub fn WdfDriverCreate(
        DriverObject: PDRIVER_OBJECT,
        RegistryPath: PCUNICODE_STRING,
        DriverAttributes: WDF_OBJECT_ATTRIBUTES,
        DriverConfig: PWDF_DRIVER_CONFIG,
    ) -> NTSTATUS;

    pub fn WDF_DRIVER_CONFIG_INIT(
        Config: PWDF_DRIVER_CONFIG,
        EvtDriverDeviceAdd: PFN_WDF_DRIVER_DEVICE_ADD,
    ) -> NTSTATUS;
}
