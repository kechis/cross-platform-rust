package com.test;

/**
 * Created by frmi
 */

public class RootUtils {

    private static native int checkRootAccessMethod1();
    private static native int checkRootAccessMethod2();
    private static native int checkRootAccessMethod3();

    public int method1() {
        return checkRootAccessMethod1();
    }
    public int method2() {
        return checkRootAccessMethod2();
    }
    public int method3() {
        return checkRootAccessMethod3();
    }
}
