extern crate cc;

fn main() {
    cc::Build::new().file("src/meat-grinder/meat_grinder.c")
        .include("/Library/Java/JavaVirtualMachines/temurin-17.jdk/Contents/Home/include")
        .include("/Library/Java/JavaVirtualMachines/temurin-17.jdk/Contents/Home/include/darwin")
        .include("/Users/frmi/Library/Android/sdk/ndk/26.2.11394342/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include")
        .include("/Users/frmi/Library/Android/sdk/ndk/26.2.11394342/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include/android")
        .include("/Users/frmi/Library/Android/sdk/ndk/26.2.11394342/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include/sys")
        .compile("root");
}
