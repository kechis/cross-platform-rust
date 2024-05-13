use std::os::raw::{c_char};
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut MG_SU_PATH: [*const c_char; 14] = [
    b"/data/local/\0" as *const u8 as *const c_char,
    b"/data/local/bin/\0" as *const u8 as *const c_char,
    b"/data/local/xbin/\0" as *const u8 as *const c_char,
    b"/sbin/\0" as *const u8 as *const c_char,
    b"/system/bin/\0" as *const u8 as *const c_char,
    b"/system/bin/.ext/\0" as *const u8 as *const c_char,
    b"/system/bin/failsafe/\0" as *const u8 as *const c_char,
    b"/system/sd/xbin/\0" as *const u8 as *const c_char,
    b"/su/xbin/\0" as *const u8 as *const c_char,
    b"/su/bin/\0" as *const u8 as *const c_char,
    b"/magisk/.core/bin/\0" as *const u8 as *const c_char,
    b"/system/usr/we-need-root/\0" as *const u8 as *const c_char,
    b"/system/xbin/\0" as *const u8 as *const c_char,
    0 as *const c_char,
];

#[no_mangle]
pub static mut MG_EXPOSED_FILES: [*const c_char; 21] = [
    b"/system/lib/libxposed_art.so\0" as *const u8 as *const c_char,
    b"/system/lib64/libxposed_art.so\0" as *const u8 as *const c_char,
    b"/system/xposed.prop\0" as *const u8 as *const c_char,
    b"/cache/recovery/xposed.zip\0" as *const u8 as *const c_char,
    b"/system/framework/XposedBridge.jar\0" as *const u8 as *const c_char,
    b"/system/bin/app_process64_xposed\0" as *const u8 as *const c_char,
    b"/system/bin/app_process32_xposed\0" as *const u8 as *const c_char,
    b"/magisk/xposed/system/lib/libsigchain.so\0" as *const u8 as *const c_char,
    b"/magisk/xposed/system/lib/libart.so\0" as *const u8 as *const c_char,
    b"/magisk/xposed/system/lib/libart-disassembler.so\0" as *const u8
        as *const c_char,
    b"/magisk/xposed/system/lib/libart-compiler.so\0" as *const u8
        as *const c_char,
    b"/system/bin/app_process32_orig\0" as *const u8 as *const c_char,
    b"/system/bin/app_process64_orig\0" as *const u8 as *const c_char,
    b"/system/lib/libmemtrack_real.so\0" as *const u8 as *const c_char,
    b"/system/lib64/libmemtrack_real.so\0" as *const u8 as *const c_char,
    b"/system/lib/libriru_edxp.so\0" as *const u8 as *const c_char,
    b"/system/lib64/libriru_edxp.so\0" as *const u8 as *const c_char,
    b"/system/lib/libwhale.edxp.so\0" as *const u8 as *const c_char,
    b"/system/lib64/libwhale.edxp.so\0" as *const u8 as *const c_char,
    b"/system/framework/edxp.jar\0" as *const u8 as *const c_char,
    0 as *const c_char,
];

#[no_mangle]
pub static mut MG_READ_ONLY_PATH: [*const c_char; 8] = [
    b"/system\0" as *const u8 as *const c_char,
    b"/system/bin\0" as *const u8 as *const c_char,
    b"/system/sbin\0" as *const u8 as *const c_char,
    b"/system/xbin\0" as *const u8 as *const c_char,
    b"/vendor/bin\0" as *const u8 as *const c_char,
    b"/sbin\0" as *const u8 as *const c_char,
    b"/etc\0" as *const u8 as *const c_char,
    0 as *const c_char,
];

#[allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_RELEASE: *const c_char = b"ro.build.version.release\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_INCREMENTAL: *const c_char = b"ro.build.version.incremental\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_CODENAME: *const c_char = b"ro.build.version.codename\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_VERSION_SDK: *const c_char = b"ro.build.version.sdk\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_MODEL: *const c_char = b"ro.product.model\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_MANUFACTURER: *const c_char = b"ro.product.manufacturer\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_BOARD: *const c_char = b"ro.product.board\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_BRAND: *const c_char = b"ro.product.brand\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_DEVICE: *const c_char = b"ro.product.device\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_PRODUCT: *const c_char = b"ro.product.name\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_HARDWARE: *const c_char = b"ro.hardware\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_CPU_ABI: *const c_char = b"ro.product.cpu.abi\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_CPU_ABI2: *const c_char = b"ro.product.cpu.abi2\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_DISPLAY: *const c_char = b"ro.build.display.id\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_HOST: *const c_char = b"ro.build.host\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_USER: *const c_char = b"ro.build.user\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_ID: *const c_char = b"ro.build.id\0" as *const u8
    as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_TYPE: *const c_char = b"ro.build.type\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_TAGS: *const c_char = b"ro.build.tags\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_FINGERPRINT: *const c_char = b"ro.build.fingerprint\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_SECURE: *const c_char = b"ro.secure\0" as *const u8
    as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_DEBUGGABLE: *const c_char = b"ro.debuggable\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_SYS_INITD: *const c_char = b"sys.initd\0" as *const u8
    as *const c_char;
#[no_mangle]
pub static mut ANDROID_OS_BUILD_SELINUX: *const c_char = b"ro.build.selinux\0"
    as *const u8 as *const c_char;
#[no_mangle]
pub static mut SERVICE_ADB_ROOT: *const c_char = b"service.adb.root\0" as *const u8
    as *const c_char;


#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    // Define constants
    const ANDROID_OS_BUILD_TAGS: &str = "ro.build.tags";
    const ANDROID_OS_DEBUGGABLE: &str = "ro.debuggable";
    const SERVICE_ADB_ROOT: &str = "service.adb.root";
    const ANDROID_OS_SECURE: &str = "ro.secure";
    const ANDROID_OS_SYS_INITD: &str = "sys.init.d";
    const ANDROID_OS_BUILD_SELINUX: &str = "ro.build.selinux";

    const MG_READ_ONLY_PATH: &[&str] = &["/system", "/vendor"];
    const MG_SU_PATH: &[&str] = &["/system/bin", "/system/xbin", "/vendor/bin", "/sbin"];
    const MG_EXPOSED_FILES: &[&str] = &[
        "/system/lib/libxposed.so",
        "/system/lib64/libxposed.so",
        "/data/data/de.robv.android.xposed.installer/bin/XposedBridge.jar",
        "/data/data/de.robv.android.xposed.installer/conf/bin/libxposed_installer.so",
    ];

    // Helper function to get system property value
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_get_system_property(key: &str) -> Option<String> {
        let c_key = CString::new(key).unwrap();
        let mut value = vec![0u8; 92];
        // let length =
        //     unsafe { __system_property_get(c_key.as_ptr(), value.as_mut_ptr() as *mut i8) };
        // if length > 0 {
        //     let value = String::from_utf8_lossy(&value[..length as usize]).into_owned();
        //     Some(value)
        // } else {
            None
        // }
    }

    // Helper function to check if a string contains a substring
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_contains_substring(haystack: &str, needle: &str) -> bool {
        haystack.contains(needle)
    }

    // Helper function to check if a string equals another string
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_equals_string(s1: &str, s2: &str) -> bool {
        s1 == s2
    }

    // Helper function to concatenate two strings
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_concat_strings(s1: &str, s2: &str) -> String {
        let mut result = String::with_capacity(s1.len() + s2.len());
        result.push_str(s1);
        result.push_str(s2);
        result
    }

    // Helper function to check if a file exists
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    // Helper function to check if a binary exists in a list of paths
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_binary_exists_in_paths(paths: &[&str], binary: &str) -> bool {
        for path in paths {
            let full_path = Java_com_gft_utils_RustRoot_concat_strings(path, binary);
            if Java_com_gft_utils_RustRoot_file_exists(&full_path) {
                return true;
            }
        }
        false
    }

    // Function to check if a mount option is present
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_present_mnt_opt(mnt_opts: &str, opt: &str) -> bool {
        let mut token = mnt_opts;
        let opt_len = opt.len();
        loop {
            if let Some(end) = token.find(',') {
                if token[..end].contains(opt) {
                    return true;
                }
                token = &token[end + 1..];
            } else {
                return token.contains(opt);
            }
        }
    }

    // Function to check if a property has a bad state
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_bad_property_state(
        key: &str,
        bad_value: &str,
        is_obligatory: bool,
        is_exact: bool,
    ) -> bool {
        if let Some(value) = Java_com_gft_utils_RustRoot_get_system_property(key) {
            if is_exact {
                Java_com_gft_utils_RustRoot_equals_string(&value, bad_value)
            } else {
                Java_com_gft_utils_RustRoot_contains_substring(&value, bad_value)
            }
        } else {
            is_obligatory
        }
    }

    // Function to check if test keys are detected
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_detected_test_keys() -> bool {
        Java_com_gft_utils_RustRoot_is_bad_property_state(ANDROID_OS_BUILD_TAGS, "test-keys", true, false)
    }

    // Function to check if dev keys are detected
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_detected_dev_keys() -> bool {
        Java_com_gft_utils_RustRoot_is_bad_property_state(ANDROID_OS_BUILD_TAGS, "dev-keys", true, false)
    }

    // Function to check if release keys are not found
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_not_found_release_keys() -> bool {
        !Java_com_gft_utils_RustRoot_is_bad_property_state(ANDROID_OS_BUILD_TAGS, "release-keys", false, true)
    }

    // Function to check if a read-only path has the "rw" mount option
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_found_wrong_path_permission() -> bool {
        let file = File::open("/proc/mounts").ok();
        if let Some(file) = file {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let mnt_dir = parts[1];
                        let mnt_opts = parts[3];
                        if MG_READ_ONLY_PATH.contains(&mnt_dir)
                            && Java_com_gft_utils_RustRoot_is_present_mnt_opt(mnt_opts, "rw")
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    // Function to check if dangerous properties are found
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_found_dangerous_props() -> bool {
        Java_com_gft_utils_RustRoot_is_bad_property_state(ANDROID_OS_DEBUGGABLE, "1", true, true)
            || Java_com_gft_utils_RustRoot_is_bad_property_state(SERVICE_ADB_ROOT, "1", false, true)
            || Java_com_gft_utils_RustRoot_is_bad_property_state(ANDROID_OS_SECURE, "0", true, true)
            || Java_com_gft_utils_RustRoot_is_bad_property_state(ANDROID_OS_SYS_INITD, "1", false, true)
    }

    // Function to check if SELinux is permissive
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_permissive_selinux() -> bool {
        Java_com_gft_utils_RustRoot_is_bad_property_state(ANDROID_OS_BUILD_SELINUX, "0", false, false)
    }

    // Function to check if the "su" binary exists
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_su_exists() -> bool {
        let output = std::process::Command::new("which")
            .arg("su")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok());
        output.is_some()
    }

    // Function to check if the Superuser.apk file exists
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_accessed_superuser_apk() -> bool {
        Java_com_gft_utils_RustRoot_file_exists("/system/app/Superuser.apk")
    }

    // Function to check if the resetprop binary exists
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_found_resetprop() -> bool {
        Java_com_gft_utils_RustRoot_file_exists("/data/magisk/resetprop")
    }

    // Function to check if the "su" binary exists in a list of paths
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_found_su_binary() -> bool {
        Java_com_gft_utils_RustRoot_binary_exists_in_paths(MG_SU_PATH, "su")
    }

    // Function to check if the "busybox" binary exists in a list of paths
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_found_busybox_binary() -> bool {
        Java_com_gft_utils_RustRoot_binary_exists_in_paths(MG_SU_PATH, "busybox")
    }

    // Function to check if any Xposed files exist
    #[no_mangle]
    pub unsafe extern fn Java_com_gft_utils_RustRoot_is_found_xposed() -> bool {
        MG_EXPOSED_FILES.iter().any(|file| Java_com_gft_utils_RustRoot_file_exists(file))
    }

    // Function to check if any hooks are found
    // #[no_mangle]
    // pub unsafe extern fn Java_com_gft_utils_RustRoot_is_found_hooks() -> bool {
    //     let pid = unsafe { getpid() };
    //     let maps_file_name = format!("/proc/{}/maps", pid);
    //     let file = File::open(&maps_file_name).ok();
    //     if let Some(file) = file {
    //         let reader = BufReader::new(file);
    //         for line in reader.lines() {
    //             if let Ok(line) = line {
    //                 if line.contains("com.saurik.substrate")
    //                     || line.contains("XposedBridge.jar")
    //                     || line.contains("edxp.jar")
    //                 {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     false
    // }
}
