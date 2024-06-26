/*
 *  Login handling
 *
 *  This Java activity handles all front-end request from the user and calls required backend functions.
 *  */

package com.notforest.sugar;

import android.annotation.SuppressLint;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.graphics.Color;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;
import android.widget.Toast;

import androidx.activity.EdgeToEdge;
import androidx.annotation.Nullable;
import androidx.appcompat.app.AppCompatActivity;

import java.util.Locale;

public class LoginActivity extends AppCompatActivity {
    // Loads external dynamic libraries.
    static {
        System.loadLibrary("sugar_jni");
    }

    // Native JNI interface for Rust backend.
    /* Handles users logins with firebase. */
    private static native int login(final String mail, final String pass);
    /* Will be used once on application startup. Logins with last token, obtained from firebase. */
    private static native String loginFast();

    // Text input values.
    EditText editTextEmail, editTextPassword;
    TextView forgotRedirect, guestLogin, signUpRedirect, mailErrorText, passwordErrorText;
    Button buttonLogin;

    /* Application's entry point.
    *
    * Since this function is being called first, it initializes all required resources, including Rust's
    * backend functions.
    * */
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        String fileDir = getFilesDir().toString();
        String cacheDir = getCacheDir().toString();
        String extFileDir = getExternalFilesDir(null).toString();
        String extCacheDir = getExternalCacheDir().toString();

        // Initialization.
        SugarInit.rustInit(fileDir, cacheDir, extFileDir, extCacheDir); // Rust initialization.

        // Visual init
        SharedPreferences prefs = getSharedPreferences("AppPrefs", Context.MODE_PRIVATE);

        int selectedColor = prefs.getInt("selectedColor", R.color.main);
        Locale chosenLocale = new Locale(prefs.getString("selectedLocale", "en".toString()));
        getResources().getConfiguration().setLocale(chosenLocale);
        getWindow().setStatusBarColor(selectedColor);

        // Trying to login via saved token credentials.
        String mail = loginFast();
        if (mail != null && !mail.isEmpty()) {
            // Jumping to login activity
            Intent i = new Intent(LoginActivity.this, MainActivity.class);
            i.putExtra("mail", mail);
            startActivity(i);
        }
    }

    @Override
    protected void onPostCreate(@Nullable Bundle savedInstanceState) {
        super.onPostCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_login);

        // Setting up the variables from environment.
        editTextEmail =             findViewById(R.id.editTextEmail);
        editTextPassword =          findViewById(R.id.editTextPassword);

        forgotRedirect =             findViewById(R.id.forgotRedirect);
        guestLogin =                findViewById(R.id.guestLogin);
        signUpRedirect =            findViewById(R.id.signUpRedirect);
        mailErrorText =             findViewById(R.id.mailErrorText);
        passwordErrorText =             findViewById(R.id.passwordErrorText);

        buttonLogin =               findViewById(R.id.buttonLogin);

        // Setting up listeners.
        buttonLogin.setOnClickListener(new View.OnClickListener() {
            @SuppressLint("SetTextI18n")
            @Override
            public void onClick(View v) {
                // Reading from fields.
                String mail = editTextEmail.getText().toString();
                String pass = editTextPassword.getText().toString();

                if (ifNotEmpty(mail, pass)) {
                    // Giving control to backend.
                    int output = login(mail, pass);
                    // Rust's backend function will return one of many defined status codes.
                    switch (output) {
                        case 0:
                            // Note about signup success.
                            Toast.makeText(
                                    LoginActivity.this,
                                    "Welcome " + mail + ". Happy reading :)))",
                                    Toast.LENGTH_SHORT
                            ).show();

                            // Jumping to main activity
                            Intent i = new Intent(LoginActivity.this, MainActivity.class);
                            i.putExtra("mail", mail);
                            startActivity(i);
                            break;
                        case 10:
                            mailErrorText.setText(R.string.error_mail_regex);
                            mailErrorText.setVisibility(View.VISIBLE);
                            break;
                        case 11:
                            passwordErrorText.setText(R.string.error_pass_wrong);
                            passwordErrorText.setVisibility(View.VISIBLE);
                            break;
                        case 30:
                            passwordErrorText.setText(R.string.error_invalid_credentials);
                            passwordErrorText.setVisibility(View.VISIBLE);
                            break;
                        case 37:
                            mailErrorText.setText(R.string.error_not_found);
                            mailErrorText.setVisibility(View.VISIBLE);
                            break;
                        case 31:
                            Toast.makeText(
                                    LoginActivity.this,
                                    "Your 'Sugar' account was disabled due to strange activity from this device. Please contact administration for further support.",
                                    Toast.LENGTH_LONG
                            ).show();
                            break;
                        default:
                            // Internal error.
                            Toast.makeText(
                                    LoginActivity.this,
                                    "Internal error has occur",
                                    Toast.LENGTH_LONG
                            ).show();
                    }
                }
            }
        });

        signUpRedirect.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // Jumping to signup activity
                startActivity(new Intent(LoginActivity.this, SignupActivity.class));
            }
        });

        guestLogin.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // Note about signup success.
                Toast.makeText(
                        LoginActivity.this,
                        "Login for better reading. Happy reading :)))",
                        Toast.LENGTH_SHORT
                ).show();

                // Jumping to login activity
                Intent i = new Intent(LoginActivity.this, MainActivity.class);
                i.putExtra("mail", "");
                startActivity(i);
            }
        });
    }

    /* Checks if both user inputs and returns true if they are. */
    private boolean ifNotEmpty(String m, String p) {
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
            mailErrorText.setText(R.string.error_pass_empty);
            passwordErrorText.setVisibility(View.VISIBLE);
            not_empty = false;
        } else {
            passwordErrorText.setVisibility(View.INVISIBLE);
        }

        return not_empty;
    }
}