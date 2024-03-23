/* Main java application entry point. */

/* This entry class defines all necessary interfaces to communicate with user and Rust backend.
 *
 * It handles the startup routine and gives control to the required activity or the backend.
 *  */

package com.notforest.sugar;

import android.os.Bundle;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {
    // Loads external dynamic libraries.
    static {
        System.loadLibrary("sugar_jni");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_main);
    }
}
