/*
 *  Login handling
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
import androidx.annotation.Nullable;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

public class LoginActivity extends AppCompatActivity {
    // Loads external dynamic libraries.
    static {
        System.loadLibrary("sugar_jni");
    }

    // Native JNI interface for Rust backend.
    /* Handles users logins with firebase. */
    private static native int login(final String mail, final String pass);

    // Text input values.
    EditText editTextEmail, editTextPassword;
    // Link to login activity.
    TextView forgotRedirect, signUpRedirect;
    Button buttonLogin;

    /* Application's entry point.
    *
    * Since this function is being called first, it initializes all required resources, including Rust's
    * backend functions.
    * */
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        // Initialization.
        SugarInit.rustInit(); // Rust initialization.
    }

    @Override
    protected void onPostCreate(@Nullable Bundle savedInstanceState) {
        super.onPostCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_login);

        // Setting up the variables from environment.
        editTextEmail =             findViewById(R.id.editTextEmail);
        editTextPassword =          findViewById(R.id.editTextPassword);
        signUpRedirect =             findViewById(R.id.signUpRedirect);
        buttonLogin =              findViewById(R.id.buttonLogin);

        // Setting up listeners.
        buttonLogin.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // Reading from fields.
                String mail = editTextEmail.getText().toString();
                String pass = editTextPassword.getText().toString();

                // Giving control to backend.
                switch (login(mail, pass)) {
                    default:

                }
            }
        });
    }
}