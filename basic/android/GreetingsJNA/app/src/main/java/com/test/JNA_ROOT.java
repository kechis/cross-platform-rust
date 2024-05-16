package com.test;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;

public interface JNA_ROOT extends Library {
    String JNA_ROOT_LIBRARY_NAME = "root";
    NativeLibrary JNA_ROOT_NATIVE_LIB = NativeLibrary.getInstance(JNA_ROOT_LIBRARY_NAME);

    JNA_ROOT ROOT_INSTANCE = (JNA_ROOT) Native.loadLibrary(JNA_ROOT_LIBRARY_NAME, JNA_ROOT.class);

    int checkRootAccessMethod1();
    int checkRootAccessMethod2();
    int checkRootAccessMethod3();
}

