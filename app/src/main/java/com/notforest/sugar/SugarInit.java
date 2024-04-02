/*
 *  Custom helper class for initialization the environment.
 * */

package com.notforest.sugar;

public class SugarInit {
    // Loads external dynamic libraries.
    static {
        System.loadLibrary("sugar_jni");
    }

    // Native JNI interface for Rust backend.
    /* Initialized all important tasks on Rust's backend side. */
    public static native void rustInit(final String filesDir, final String cacheDir, final String extFilesDir, final String extCacheDir);
}
