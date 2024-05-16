package com.test;

import com.test.JNA_ROOT;

public class RootUtils {

    public int checkRootAccessMethod1() {
        return JNA_ROOT.ROOT_INSTANCE.checkRootAccessMethod1();
    }

    public int checkRootAccessMethod2() {
        return JNA_ROOT.ROOT_INSTANCE.checkRootAccessMethod2();
    }

    public int checkRootAccessMethod3() {
        return JNA_ROOT.ROOT_INSTANCE.checkRootAccessMethod3();
    }
}
