#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut MG_SU_PATH: [*const libc::c_char; 14] = [
    b"/data/local/\0" as *const u8 as *const libc::c_char,
    b"/data/local/bin/\0" as *const u8 as *const libc::c_char,
    b"/data/local/xbin/\0" as *const u8 as *const libc::c_char,
    b"/sbin/\0" as *const u8 as *const libc::c_char,
    b"/system/bin/\0" as *const u8 as *const libc::c_char,
    b"/system/bin/.ext/\0" as *const u8 as *const libc::c_char,
    b"/system/bin/failsafe/\0" as *const u8 as *const libc::c_char,
    b"/system/sd/xbin/\0" as *const u8 as *const libc::c_char,
    b"/su/xbin/\0" as *const u8 as *const libc::c_char,
    b"/su/bin/\0" as *const u8 as *const libc::c_char,
    b"/magisk/.core/bin/\0" as *const u8 as *const libc::c_char,
    b"/system/usr/we-need-root/\0" as *const u8 as *const libc::c_char,
    b"/system/xbin/\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut MG_EXPOSED_FILES: [*const libc::c_char; 21] = [
    b"/system/lib/libxposed_art.so\0" as *const u8 as *const libc::c_char,
    b"/system/lib64/libxposed_art.so\0" as *const u8 as *const libc::c_char,
    b"/system/xposed.prop\0" as *const u8 as *const libc::c_char,
    b"/cache/recovery/xposed.zip\0" as *const u8 as *const libc::c_char,
    b"/system/framework/XposedBridge.jar\0" as *const u8 as *const libc::c_char,
    b"/system/bin/app_process64_xposed\0" as *const u8 as *const libc::c_char,
    b"/system/bin/app_process32_xposed\0" as *const u8 as *const libc::c_char,
    b"/magisk/xposed/system/lib/libsigchain.so\0" as *const u8 as *const libc::c_char,
    b"/magisk/xposed/system/lib/libart.so\0" as *const u8 as *const libc::c_char,
    b"/magisk/xposed/system/lib/libart-disassembler.so\0" as *const u8
        as *const libc::c_char,
    b"/magisk/xposed/system/lib/libart-compiler.so\0" as *const u8
        as *const libc::c_char,
    b"/system/bin/app_process32_orig\0" as *const u8 as *const libc::c_char,
    b"/system/bin/app_process64_orig\0" as *const u8 as *const libc::c_char,
    b"/system/lib/libmemtrack_real.so\0" as *const u8 as *const libc::c_char,
    b"/system/lib64/libmemtrack_real.so\0" as *const u8 as *const libc::c_char,
    b"/system/lib/libriru_edxp.so\0" as *const u8 as *const libc::c_char,
    b"/system/lib64/libriru_edxp.so\0" as *const u8 as *const libc::c_char,
    b"/system/lib/libwhale.edxp.so\0" as *const u8 as *const libc::c_char,
    b"/system/lib64/libwhale.edxp.so\0" as *const u8 as *const libc::c_char,
    b"/system/framework/edxp.jar\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut MG_READ_ONLY_PATH: [*const libc::c_char; 8] = [
    b"/system\0" as *const u8 as *const libc::c_char,
    b"/system/bin\0" as *const u8 as *const libc::c_char,
    b"/system/sbin\0" as *const u8 as *const libc::c_char,
    b"/system/xbin\0" as *const u8 as *const libc::c_char,
    b"/vendor/bin\0" as *const u8 as *const libc::c_char,
    b"/sbin\0" as *const u8 as *const libc::c_char,
    b"/etc\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
