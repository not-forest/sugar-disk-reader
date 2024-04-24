/*
 *  Profile fragment for user's profile management.
 *
 *  From here, users can change their emails, passwords and logout.
 *  */

package com.notforest.sugar.ui.profile;


import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.AdapterView;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.Spinner;
import android.widget.TextView;
import android.widget.Toast;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;
import androidx.lifecycle.ViewModelProvider;

import com.notforest.sugar.LoginActivity;
import com.notforest.sugar.R;
import com.notforest.sugar.MainActivity;
import com.notforest.sugar.databinding.FragmentHomeBinding;
import com.notforest.sugar.databinding.FragmentProfileBinding;
import com.notforest.sugar.ui.home.HomeViewModel;


public class ProfileFragment extends Fragment {
    static {
        System.loadLibrary("sugar_jni");
    }

    /* Method for changing user email.  */
    private static native int changeEmail(final String email);
    /* Method for changing user password. */
    private static native int changePassword(final String pass_old, final String pass_new);
    /* Logging out from user's profile. */
    private static native int logout();

    private LinearLayout changeEmailLayout, changePasswordLayout, logoutLayout;
    private Button changeEmailButton, changePasswordButton, logoutButton;
    private FragmentProfileBinding binding;
    private TextView[] textViews;
    private TextView userMail;
    private ImageView userPFP;
    private View root;

    public View onCreateView(@NonNull LayoutInflater inflater,
                             ViewGroup container, Bundle savedInstanceState) {
        HomeViewModel homeViewModel =
                new ViewModelProvider(this).get(HomeViewModel.class);

        binding = FragmentProfileBinding.inflate(inflater, container, false);
        root = binding.getRoot();

        // Access the activity associated with the fragment
        MainActivity mainActivity = (MainActivity) getActivity();
        if (mainActivity != null) {
            int randomIndex = (int) (Math.random() * mainActivity.backgroundDrawables.length);
            root.setBackgroundResource(mainActivity.backgroundDrawables[randomIndex]);

            Spinner spinner = root.findViewById(R.id.profile_management_spinner);

            changeEmailLayout = root.findViewById(R.id.change_email_layout);
            changePasswordLayout = root.findViewById(R.id.change_password_layout);
            logoutLayout = root.findViewById(R.id.logout_layout);

            changePasswordButton = root.findViewById(R.id.change_password_button);
            changeEmailButton = root.findViewById(R.id.change_email_button);
            logoutButton = root.findViewById(R.id.logout_button);

            spinner.setOnItemSelectedListener(new AdapterView.OnItemSelectedListener() {
                @Override
                public void onItemSelected(AdapterView<?> parent, View view, int position, long id) {
                    switch (position) {
                        case 0: // Change Email selected
                            changeEmailLayout.setVisibility(View.VISIBLE);
                            changePasswordLayout.setVisibility(View.GONE);
                            logoutLayout.setVisibility(View.GONE);

                            changeEmailButton.setVisibility(View.VISIBLE);
                            changePasswordButton.setVisibility(View.GONE);
                            logoutButton.setVisibility(View.GONE);
                            break;
                        case 1: // Change Password selected
                            changeEmailLayout.setVisibility(View.GONE);
                            changePasswordLayout.setVisibility(View.VISIBLE);
                            logoutLayout.setVisibility(View.GONE);

                            changeEmailButton.setVisibility(View.GONE);
                            changePasswordButton.setVisibility(View.VISIBLE);
                            logoutButton.setVisibility(View.GONE);
                            break;
                        case 2: // Logout selected
                            changeEmailLayout.setVisibility(View.GONE);
                            changePasswordLayout.setVisibility(View.GONE);
                            logoutLayout.setVisibility(View.VISIBLE);

                            changeEmailButton.setVisibility(View.GONE);
                            changePasswordButton.setVisibility(View.GONE);
                            logoutButton.setVisibility(View.VISIBLE);
                            break;
                    }
                }

                @Override
                public void onNothingSelected(AdapterView<?> parent) {
                    // Hide all layouts if nothing is selected
                    changeEmailLayout.setVisibility(View.GONE);
                    changePasswordLayout.setVisibility(View.GONE);
                    logoutButton.setVisibility(View.GONE);
                }
            });

            changeEmailButton.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View v) {
                    TextView new_mail = root.findViewById(R.id.new_email_input);

                    changeEmail(new_mail.getText().toString());
                }
            });

            changePasswordButton.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View v) {
                    TextView old_pass = root.findViewById(R.id.old_password_input);
                    TextView new_pass = root.findViewById(R.id.new_password_input);

                    changePassword(
                            old_pass.getText().toString(),
                            new_pass.getText().toString()
                    );
                }
            });

            logoutButton.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View v) {
                    switch (logout()) {
                        case 0:
                            // We are now logged out, so jumping to login screen.
                        case 40:
                            // No file means the user is not logged in for some reason.
                            mainActivity.finish();
                            break;
                        case 46:
                            // IO interrupted, trying one more time.
                            logout();
                            break;
                        default:
                            Toast.makeText(
                                    mainActivity,
                                    "Unexpected error has occur. Please try again.",
                                    Toast.LENGTH_SHORT
                            ).show();
                    }
                }
            });

            userMail = root.findViewById(R.id.profile_user_mail);
            userPFP = root.findViewById(R.id.profile_user_pfp);

            // Get intent and update UI elements
            Intent i = mainActivity.getIntent();
            String m = i.getStringExtra("mail");
            if (m != null && !m.isEmpty()) {
                userMail.setText(m);
                // Fetch the profile picture on a background thread
                mainActivity.fetchProfilePicture(m, userPFP);
            }

            // Create an array of TextViews
            textViews = new TextView[] {
                    root.findViewById(R.id.change_email),
                    root.findViewById(R.id.change_password),
                    root.findViewById(R.id.logout)
            };

            // Populate the Spinner with TextViews using the adapter
            ProfileManagementAdapter adapter = new ProfileManagementAdapter(requireContext(), textViews);
            spinner.setAdapter(adapter);
        }

        return root;
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        root.setBackgroundResource(0);
        binding = null;
    }
}

class ProfileManagementAdapter extends ArrayAdapter<TextView> {
    private final LayoutInflater inflater;
    private boolean isFirstClick = true;

    public ProfileManagementAdapter(Context context, TextView[] options) {
        super(context, R.layout.dropdown_item, options);
        inflater = LayoutInflater.from(context);
    }

    @Override
    @NonNull
    public View getView(int position, View convertView, @NonNull ViewGroup parent) {
        View view = convertView;
        if (view == null) {
            view = inflater.inflate(R.layout.dropdown_item, parent, false);
        }
        TextView textView = getItem(position);
        if (textView != null) {
            TextView spinnerTextView = view.findViewById(R.id.spinner_item_text);
            if (isFirstClick && position == 0) {
                spinnerTextView.setText(R.string.manage_profile);
            } else {
                spinnerTextView.setText(textView.getText());
            }
        }
        return view;
    }

    @Override
    public View getDropDownView(int position, View convertView, @NonNull ViewGroup parent) {
        isFirstClick = false; // User has clicked, so set isFirstClick to false
        return getView(position, convertView, parent);
    }
}


