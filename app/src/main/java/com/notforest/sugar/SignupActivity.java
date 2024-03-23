/*
 *  Sign Up handling
 *
 *  This Java activity handles all front-end request from the user and calls required backend functions.
 *  */

package com.notforest.sugar;

import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;

public class SignupActivity extends AppCompatActivity {
    // Loads external dynamic libraries.
    static {
        System.loadLibrary("sugar_jni");
    }

    // Native JNI interface for Rust backend.
    /* Handles registration with firebase. */
    private static native int signUp(final String mail, final String pass, final String conf);

    // Text input values.
    EditText editTextEmail, editTextPassword, editTextConfirmPassword;
    // Link to login activity.
    TextView loginRedirect;
    Button buttonSignUp;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_signup);
        // Setting up the variables from environment.
        editTextEmail =             findViewById(R.id.editTextEmail);
        editTextPassword =          findViewById(R.id.editTextPassword);
        editTextConfirmPassword =   findViewById(R.id.editTextConfirmPassword);
        loginRedirect =             findViewById(R.id.loginRedirect);
        buttonSignUp =              findViewById(R.id.buttonSignUp);

        // Setting up listeners.
        buttonSignUp.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // Reading from fields.
                String mail = editTextEmail.getText().toString();
                String pass = editTextPassword.getText().toString();
                String conf = editTextConfirmPassword.getText().toString();

                if (pass.equals(conf)) {
                    // Giving control to backend.
                    signUp(mail, pass, conf);
                }
            }
        });
    }
}
