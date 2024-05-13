#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_RELEASE: *const libc::c_char = b"ro.build.version.release\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_INCREMENTAL: *const libc::c_char = b"ro.build.version.incremental\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_CODENAME: *const libc::c_char = b"ro.build.version.codename\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_SDK: *const libc::c_char = b"ro.build.version.sdk\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_MODEL: *const libc::c_char = b"ro.product.model\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_MANUFACTURER: *const libc::c_char = b"ro.product.manufacturer\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_BOARD: *const libc::c_char = b"ro.product.board\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_BRAND: *const libc::c_char = b"ro.product.brand\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_DEVICE: *const libc::c_char = b"ro.product.device\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_PRODUCT: *const libc::c_char = b"ro.product.name\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_HARDWARE: *const libc::c_char = b"ro.hardware\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_CPU_ABI: *const libc::c_char = b"ro.product.cpu.abi\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_CPU_ABI2: *const libc::c_char = b"ro.product.cpu.abi2\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_DISPLAY: *const libc::c_char = b"ro.build.display.id\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_HOST: *const libc::c_char = b"ro.build.host\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_USER: *const libc::c_char = b"ro.build.user\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_ID: *const libc::c_char = b"ro.build.id\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_TYPE: *const libc::c_char = b"ro.build.type\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_TAGS: *const libc::c_char = b"ro.build.tags\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_FINGERPRINT: *const libc::c_char = b"ro.build.fingerprint\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_SECURE: *const libc::c_char = b"ro.secure\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_DEBUGGABLE: *const libc::c_char = b"ro.debuggable\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_SYS_INITD: *const libc::c_char = b"sys.initd\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_SELINUX: *const libc::c_char = b"ro.build.selinux\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut SERVICE_ADB_ROOT: *const libc::c_char = b"service.adb.root\0" as *const u8
    as *const libc::c_char;
