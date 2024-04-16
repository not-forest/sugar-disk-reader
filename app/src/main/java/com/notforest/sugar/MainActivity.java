package com.notforest.sugar;

import android.content.Intent;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.util.Log;
import android.view.View;
import android.view.Menu;
import android.widget.ImageView;
import android.widget.TextView;

import com.google.android.material.snackbar.Snackbar;
import com.google.android.material.navigation.NavigationView;

import androidx.navigation.NavController;
import androidx.navigation.Navigation;
import androidx.navigation.ui.AppBarConfiguration;
import androidx.navigation.ui.NavigationUI;
import androidx.drawerlayout.widget.DrawerLayout;
import androidx.appcompat.app.AppCompatActivity;

import android.graphics.Bitmap;
import android.graphics.BitmapFactory;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.io.IOException;
import java.io.InputStream;
import java.net.HttpURLConnection;
import java.net.URL;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import com.notforest.sugar.databinding.ActivityMainBinding;

public class MainActivity extends AppCompatActivity {

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
        TextView userMail = headerView.findViewById(R.id.user_mail);
        ImageView userPFP = headerView.findViewById(R.id.user_pfp);
        TextView customText = headerView.findViewById(R.id.custom_text);

        // Get intent and update UI elements
        Intent i = getIntent();
        String m = i.getStringExtra("mail");
        if (m != null && !m.isEmpty()) {
            userMail.setText(m);
            customText.setText(R.string.greetings_user);
            // Fetch the profile picture on a background thread
            fetchProfilePicture(m, userPFP);
        }

        // Setup the rest of your navigation and drawer behavior
        mAppBarConfiguration = new AppBarConfiguration.Builder(
                R.id.nav_home, R.id.nav_settings, R.id.nav_profile)
                .setOpenableLayout(drawer)
                .build();
        NavController navController = Navigation.findNavController(this, R.id.nav_host_fragment_content_main);
        NavigationUI.setupActionBarWithNavController(this, navController, mAppBarConfiguration);
        NavigationUI.setupWithNavController(navigationView, navController);
    }

    private void fetchProfilePicture(String email, ImageView profileImageView) {
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
            } catch (IOException e) {
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
            return bigInt.toString(16);
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
            return null;
        }
    }

    public static Bitmap getProfilePicture(String email) throws IOException {
        // Construct the URL for the profile image
        String imageUrl = "https://lh3.googleusercontent.com/a/ACg8ocJiNEfzbvCMIcJRsuK9MXQqdvobCXVB3I5p9xPWeJbuqfrJ0JA=s288-c-no";

        // Open a connection to the URL
        URL url = new URL(imageUrl);
        HttpURLConnection connection = (HttpURLConnection) url.openConnection();
        connection.setDoInput(true);
        connection.connect();

        // Get the input stream containing the image data
        InputStream input = connection.getInputStream();

        // Decode the input stream into a Bitmap
        return BitmapFactory.decodeStream(input);
    }

}