#include <string.h>
#include <jni.h>
#include <time.h>
#include <sys/stat.h>
#include <stdio.h>
#include "android_log.h"
#include <errno.h>
#include <unistd.h>
#include <sys/system_properties.h>

#define ANDROID_OS_BUILD_VERSION_RELEASE     "ro.build.version.release"          // * The user-visible version string. E.g., "1.0" or "3.4b5".
#define ANDROID_OS_BUILD_VERSION_INCREMENTAL "ro.build.version.incremental"      // The internal value used by the underlying source control to represent this build.
#define ANDROID_OS_BUILD_VERSION_CODENAME    "ro.build.version.codename"         // The current development codename, or the string "REL" if this is a release build.
#define ANDROID_OS_BUILD_VERSION_SDK         "ro.build.version.sdk"              // The user-visible SDK version of the framework.

#define ANDROID_OS_BUILD_MODEL               "ro.product.model"                  // * The end-user-visible name for the end product..
#define ANDROID_OS_BUILD_MANUFACTURER        "ro.product.manufacturer"           // The manufacturer of the product/hardware.
#define ANDROID_OS_BUILD_BOARD               "ro.product.board"                  // The name of the underlying board, like "goldfish".
#define ANDROID_OS_BUILD_BRAND               "ro.product.brand"                  // The brand (e.g., carrier) the software is customized for, if any.
#define ANDROID_OS_BUILD_DEVICE              "ro.product.device"                 // The name of the industrial design.
#define ANDROID_OS_BUILD_PRODUCT             "ro.product.name"                   // The name of the overall product.
#define ANDROID_OS_BUILD_HARDWARE            "ro.hardware"                       // The name of the hardware (from the kernel command line or /proc).
#define ANDROID_OS_BUILD_CPU_ABI             "ro.product.cpu.abi"                // The name of the instruction set (CPU type + ABI convention) of native code.
#define ANDROID_OS_BUILD_CPU_ABI2            "ro.product.cpu.abi2"               // The name of the second instruction set (CPU type + ABI convention) of native code.

#define ANDROID_OS_BUILD_DISPLAY             "ro.build.display.id"               // * A build ID string meant for displaying to the user.
#define ANDROID_OS_BUILD_HOST                "ro.build.host"
#define ANDROID_OS_BUILD_USER                "ro.build.user"
#define ANDROID_OS_BUILD_ID                  "ro.build.id"                       // Either a changelist number, or a label like "M4-rc20".
#define ANDROID_OS_BUILD_TYPE                "ro.build.type"                     // The type of build, like "user" or "eng".
#define ANDROID_OS_BUILD_TAGS                "ro.build.tags"                     // Comma-separated tags describing the build, like "unsigned,debug".

#define ANDROID_OS_BUILD_FINGERPRINT         "ro.build.fingerprint"              // A string that uniquely identifies this build. 'BRAND/PRODUCT/DEVICE:RELEASE/ID/VERSION.INCREMENTAL:TYPE/TAGS'.


/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_test_RootUtils_checkRootAccessMethod1(env: JNIEnv, _: JClass) -> jstring {
        //Access function checks whether a particular file can be accessed
        int result = access("/system/app/Superuser.apk",F_OK);

        ANDROID_LOGV( "File Access Result %d\n", result);

        int len;
        char build_tags[PROP_VALUE_MAX]; // PROP_VALUE_MAX from <sys/system_properties.h>.
        len = __system_property_get(ANDROID_OS_BUILD_TAGS, build_tags); // On return, len will equal (int)strlen(model_id).
        if(strcmp(build_tags,"test-keys") == 0){
            ANDROID_LOGV( "Device has test keys\n", build_tags);
            result = 0;
        }
        ANDROID_LOGV( "File Access Result %s\n", build_tags);
        return result;

    }

    #[no_mangle]
    pub unsafe extern fn Java_com_test_RootUtils_checkRootAccessMethod2(env: JNIEnv, _: JClass) -> jstring {
        //which command is enabled only after Busy box is installed on a rooted device
        //Outpput of which command is the path to su file. On a non rooted device , we will get a null/ empty path
        //char* cmd = const_cast<char *>"which su";
        FILE* pipe = popen("which su", "r");
        if (!pipe) return -1;
        char buffer[128];
        std::string resultCmd = "";
        while(!feof(pipe)) {
            if(fgets(buffer, 128, pipe) != NULL)
                resultCmd += buffer;
        }
        pclose(pipe);

        const char *cstr = resultCmd.c_str();
        int result = -1;
        if(cstr == NULL || (strlen(cstr) == 0)){
            ANDROID_LOGV( "Result of Which command is Null");
        }else{
            result = 0;
            ANDROID_LOGV( "Result of Which command %s\n", cstr);
            }
        return result;

    }


    #[no_mangle]
    pub unsafe extern fn Java_com_test_RootUtils_checkRootAccessMethod3(env: JNIEnv, _: JClass) -> jstring {
        int len;
        char build_tags[PROP_VALUE_MAX]; // PROP_VALUE_MAX from <sys/system_properties.h>.
        int result = -1;
        len = __system_property_get(ANDROID_OS_BUILD_TAGS, build_tags); // On return, len will equal (int)strlen(model_id).
        if(len >0 && strstr(build_tags,"test-keys") != NULL){
            ANDROID_LOGV( "Device has test keys\n", build_tags);
            result = 0;
        }

        return result;

    }

}
