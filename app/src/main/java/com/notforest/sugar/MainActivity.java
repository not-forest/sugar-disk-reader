/*
 *  Main application activity.
 *
 *  This activity handles all three main fragments, which are the key part of the application
 *  as a whole:
 *  - Home;
 *  - Settings;
 *  - Profile;
 *
 *  Activity allows for real time connection with target device, as well as some minor customization
 *  of the front-end part.
 *  */

package com.notforest.sugar;

import android.content.Context;
import android.content.Intent;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.view.View;
import android.view.Menu;
import android.widget.ImageView;
import android.widget.TextView;

import com.google.android.material.navigation.NavigationView;

import androidx.annotation.Nullable;
import androidx.appcompat.app.AppCompatActivity;
import androidx.drawerlayout.widget.DrawerLayout;
import androidx.fragment.app.Fragment;
import androidx.fragment.app.FragmentManager;
import androidx.navigation.NavController;
import androidx.navigation.Navigation;
import androidx.navigation.ui.AppBarConfiguration;
import androidx.navigation.ui.NavigationUI;

import java.io.IOException;
import java.io.InputStream;
import java.math.BigInteger;
import java.net.HttpURLConnection;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import com.notforest.sugar.databinding.ActivityMainBinding;

public class MainActivity extends AppCompatActivity {

    public final int[] backgroundDrawables = {R.drawable.bg0, R.drawable.bg1, R.drawable.bg2,
            R.drawable.bg3, R.drawable.bg4, R.drawable.bg5, R.drawable.bg6};
    private AppBarConfiguration mAppBarConfiguration;
    private ActivityMainBinding binding;
    private TextView userMail, customText;
    private ImageView userPFP;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        setSupportActionBar(binding.appBarMain.toolbar);

        // Setup the drawer and navigation view
        DrawerLayout drawer = binding.drawerLayout;
        NavigationView navigationView = binding.navView;

        // Access the header view from the navigation view
        View headerView = navigationView.getHeaderView(0);
        userMail = headerView.findViewById(R.id.user_mail);
        userPFP = headerView.findViewById(R.id.user_pfp);
        customText = headerView.findViewById(R.id.custom_text);

        // Call the method to change user data
        Intent i = getIntent();
        String m = i.getStringExtra("mail");
        change_user_data(m);

        // Setup the rest of your navigation and drawer behavior
        mAppBarConfiguration = new AppBarConfiguration.Builder(
                R.id.nav_home, R.id.nav_settings, R.id.nav_profile)
                .setOpenableLayout(drawer)
                .build();
        NavController navController = Navigation.findNavController(this, R.id.nav_host_fragment_content_main);
        NavigationUI.setupActionBarWithNavController(this, navController, mAppBarConfiguration);
        NavigationUI.setupWithNavController(navigationView, navController);
    }

    @Override
    protected void onPostCreate(@Nullable Bundle savedInstanceState) {
        super.onPostCreate(savedInstanceState);

        int selectedColor = getSharedPreferences("AppPrefs", Context.MODE_PRIVATE)
                .getInt("selectedColor", R.color.main);
        androidx.appcompat.widget.Toolbar toolbar = findViewById(R.id.toolbar);
        getWindow().setStatusBarColor(selectedColor);
        toolbar.setBackgroundColor(selectedColor);
    }

    public void change_user_data(String m) {
        // Get intent and update UI elements
        if (m != null && !m.isEmpty()) {
            userMail.setText(m);
            customText.setText(R.string.greetings_user);
            // Fetch the profile picture on a background thread
            fetchProfilePicture(m, userPFP);
        }
    }

    public void fetchProfilePicture(String email, ImageView profileImageView) {
        ExecutorService executor = Executors.newSingleThreadExecutor();
        Handler handler = new Handler(Looper.getMainLooper());
        executor.execute(() -> {
            try {
                final Bitmap profilePicture = getProfilePicture(email);
                handler.post(() -> {
                    if (profilePicture != null) {
                        profileImageView.setImageBitmap(profilePicture);
                    }
                });
            } catch (Exception e) {
                e.printStackTrace();
            }
        });
        executor.shutdown();
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.main, menu);
        return true;
    }

    @Override
    public boolean onSupportNavigateUp() {
        NavController navController = Navigation.findNavController(this, R.id.nav_host_fragment_content_main);
        return NavigationUI.navigateUp(navController, mAppBarConfiguration)
                || super.onSupportNavigateUp();
    }

    public static String getEmailHash(String email) {
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            md.update(email.toLowerCase().trim().getBytes(StandardCharsets.UTF_8));
            byte[] digest = md.digest();
            BigInteger bigInt = new BigInteger(1, digest);
            // Ensure the hash is in the form of a 32-digit hexadecimal number
            return String.format("%032x", bigInt);
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
            return null;
        }
    }

    public static Bitmap getProfilePicture(String email) {
        String hash = getEmailHash(email);
        if (hash == null) {
            return null; // Handle case where hashing fails
        }

        String imageUrl = "https://lh3.googleusercontent.com/a/" + hash + "=s288-c-no";
        HttpURLConnection connection = null;
        InputStream input = null;
        try {
            URL url = new URL(imageUrl);
            connection = (HttpURLConnection) url.openConnection();
            connection.setDoInput(true);
            connection.setConnectTimeout(5000); // 5 seconds connect timeout
            connection.setReadTimeout(5000); // 5 seconds read timeout
            connection.connect();

            input = connection.getInputStream();
            return BitmapFactory.decodeStream(input);
        } catch (IOException e) {
            e.printStackTrace();
            return null;
        } finally {
            try {
                if (input != null) input.close();
                if (connection != null) connection.disconnect();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }
}
