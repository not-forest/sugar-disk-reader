/*
 *  Sign Up handling
 *
 *  This Java activity handles all front-end request from the user and calls required backend functions.
 *  */

package com.notforest.sugar;

import android.content.Intent;
import android.os.Bundle;
import android.view.KeyEvent;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;
import android.widget.Toast;

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
    TextView loginRedirect, mailErrorText, passwordErrorText, confirmPasswordErrorText;
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

        mailErrorText =             findViewById(R.id.mailErrorText);
        passwordErrorText =         findViewById(R.id.passwordErrorText);
        confirmPasswordErrorText =  findViewById(R.id.confirmPasswordErrorText);

        // Setting up listeners.
        buttonSignUp.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                mailErrorText.setVisibility(View.INVISIBLE);
                passwordErrorText.setVisibility(View.INVISIBLE);
                confirmPasswordErrorText.setVisibility(View.INVISIBLE);

                // Reading from fields.
                String mail = editTextEmail.getText().toString();
                String pass = editTextPassword.getText().toString();
                String conf = editTextConfirmPassword.getText().toString();

                if (pass.equals(conf)) {
                    if (ifNotEmpty(mail, pass, conf)) {
                        // Giving control to backend.
                        int output = signUp(mail, pass, conf);
                        // Rust's backend function will return one of many defined status codes.
                        switch (output) {
                            case 0:
                                // Note about signup success.
                                Toast.makeText(
                                        SignupActivity.this,
                                        "Created account successfully! Please login.",
                                        Toast.LENGTH_SHORT
                                ).show();

                                // Jumping to login activity
                                startActivity(new Intent(SignupActivity.this, LoginActivity.class));
                                break;
                            case 10:
                                mailErrorText.setText(R.string.error_mail_regex);
                                mailErrorText.setVisibility(View.VISIBLE);
                                break;
                            case 11:
                                passwordErrorText.setText(R.string.error_pass_weak);
                                passwordErrorText.setVisibility(View.VISIBLE);
                                break;
                            case 20:
                                mailErrorText.setText(R.string.error_mail_used);
                                mailErrorText.setVisibility(View.VISIBLE);
                                break;
                            case 21:
                                Toast.makeText(
                                        SignupActivity.this,
                                        "Your 'Sugar' account was disabled due to strange activity from this device. Please contact administration for further support.",
                                        Toast.LENGTH_LONG
                                ).show();
                                break;
                            default:
                                // Internal error.
                                Toast.makeText(
                                        SignupActivity.this,
                                        "Internal error has occur",
                                        Toast.LENGTH_LONG
                                ).show();
                        }
                    }
                } else {
                    confirmPasswordErrorText.setText(R.string.error_pass_nomatch);
                    confirmPasswordErrorText.setVisibility(View.VISIBLE);
                }
            }
        });

        loginRedirect.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // Jumping to signup activity
                startActivity(new Intent(SignupActivity.this, LoginActivity.class));
            }
        });
    }

    /* Checks if all user inputs and returns true if they are. */
    private boolean ifNotEmpty(String m, String p, String f) {
        boolean not_empty = true;

        // Mail field.
        if (m.isEmpty()) {
            mailErrorText.setText(R.string.error_mail_empty);
            mailErrorText.setVisibility(View.VISIBLE);
            not_empty = false;
        } else {
            mailErrorText.setVisibility(View.INVISIBLE);
        }

        // Password field.
        if (p.isEmpty()) {
            passwordErrorText.setText(R.string.error_pass_empty);
            passwordErrorText.setVisibility(View.VISIBLE);
            not_empty = false;
        } else {
            passwordErrorText.setVisibility(View.INVISIBLE);
        }

        // Password field.
        if (f.isEmpty()) {
            confirmPasswordErrorText.setText(R.string.error_confirm_password_empty);
            confirmPasswordErrorText.setVisibility(View.VISIBLE);
            not_empty = false;
        } else {
            confirmPasswordErrorText.setVisibility(View.INVISIBLE);
        }

        return not_empty;
    }
}
