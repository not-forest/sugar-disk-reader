/*
 *  Main Home fragment.
 *
 *  Handles all front-end IO communication with the user, when it comes to creating and managing
 *  targets for future reading. This fragment will also allow for real time communication with
 *  chosen architecture and provide parsed results from reading files.
 * */

package com.notforest.sugar.ui.home;

import android.app.AlertDialog;
import android.content.DialogInterface;
import android.content.Intent;
import android.content.res.Resources;
import android.graphics.Color;
import android.os.Bundle;
import android.view.Gravity;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.ImageButton;
import android.widget.LinearLayout;
import android.widget.TextView;
import android.widget.Toast;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.cardview.widget.CardView;
import androidx.fragment.app.DialogFragment;
import androidx.fragment.app.Fragment;
import androidx.lifecycle.ViewModelProvider;
import androidx.navigation.NavDirections;
import androidx.navigation.Navigation;

import com.google.android.material.floatingactionbutton.FloatingActionButton;
import com.notforest.sugar.MainActivity;
import com.notforest.sugar.R;
import com.notforest.sugar.databinding.FragmentHomeBinding;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;

public class HomeFragment extends Fragment {

    private View root;
    private FloatingActionButton addButton;
    private LinearLayout machineListLayout;

    public View onCreateView(@NonNull LayoutInflater inflater,
                             ViewGroup container, Bundle savedInstanceState) {
        HomeViewModel homeViewModel =
                new ViewModelProvider(this).get(HomeViewModel.class);

        root = inflater.inflate(R.layout.fragment_home, container, false);
        MainActivity mainActivity = (MainActivity) getActivity();
        if (mainActivity != null) {
            int randomIndex = (int) (Math.random() * mainActivity.backgroundDrawables.length);
            root.setBackgroundResource(mainActivity.backgroundDrawables[randomIndex]);
        }

        addButton = root.findViewById(R.id.add_button);
        machineListLayout = root.findViewById(R.id.machineListLayout);

        addButton.setOnClickListener(v -> openAddArchitectureFragment());

        loadMachinesFromDirectory();

        return root;
    }

    private void openAddArchitectureFragment() {
        AddArchitectureFragment dialogFragment = new AddArchitectureFragment();
        dialogFragment.show(getParentFragmentManager(), "AddArchitectureFragment");
    }

    private void loadMachinesFromDirectory() {
        File[] architectureDirs = getContext().getFilesDir().listFiles();
        if (architectureDirs != null) {
            for (File architectureDir : architectureDirs) {
                File[] machineFiles = architectureDir.listFiles();
                if (machineFiles != null && machineFiles.length > 0) {
                    root.findViewById(R.id.cardView).setVisibility(View.INVISIBLE);
                    for (File machineFile : machineFiles) {
                        try {
                            displayMachineDetails(machineFile);
                        } catch (IOException | JSONException e) {
                            e.printStackTrace();
                            showErrorToast();
                        }
                    }
                } else {
                    root.findViewById(R.id.cardView).setVisibility(View.VISIBLE);
                }
            }
        }
    }

    private void displayMachineDetails(File machineFile) throws IOException, JSONException {
        FileInputStream inputStream = new FileInputStream(machineFile);
        int size = inputStream.available();
        byte[] buffer = new byte[size];
        inputStream.read(buffer);
        inputStream.close();
        String json = new String(buffer, "UTF-8");

        JSONObject machineJson = new JSONObject(json);
        String machineName = machineJson.optString("name");
        String machineArchitecture = machineJson.optString("architecture");
        String machineNotes = machineJson.optString("notes");

        CardView cardView = createCardView();
        LinearLayout cardContentLayout = createCardContentLayout();
        TextView machineTextView = createMachineTextView(machineName, machineArchitecture, machineNotes);
        ImageButton closeButton = createCloseButton(machineFile, machineName);

        cardContentLayout.addView(machineTextView);
        cardContentLayout.addView(closeButton);
        cardView.addView(cardContentLayout);
        cardView.setOnClickListener(v -> openMachineDetailsFragment(machineFile));
        machineListLayout.addView(cardView);
    }

    private CardView createCardView() {
        CardView cardView = new CardView(getContext());
        LinearLayout.LayoutParams cardParams = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
        );
        cardParams.setMargins(0, 0, 0, 20);
        cardView.setLayoutParams(cardParams);
        cardView.setRadius(45);
        cardView.setCardElevation(20);
        cardView.setCardBackgroundColor(getResources().getColor(R.color.main_transparent));
        return cardView;
    }

    private LinearLayout createCardContentLayout() {
        LinearLayout cardContentLayout = new LinearLayout(getContext());
        cardContentLayout.setLayoutParams(new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
        ));
        cardContentLayout.setOrientation(LinearLayout.HORIZONTAL);
        cardContentLayout.setGravity(Gravity.END);
        return cardContentLayout;
    }

    private TextView createMachineTextView(String machineName, String machineArchitecture, String machineNotes) {
        TextView machineTextView = new TextView(getContext());
        LinearLayout.LayoutParams textParams = new LinearLayout.LayoutParams(
                0,
                LinearLayout.LayoutParams.WRAP_CONTENT,
                1.0f
        );
        textParams.setMargins(50, 20, 0, 20);
        machineTextView.setLayoutParams(textParams);
        StringBuilder displayText = new StringBuilder();
        displayText.append("Name: ").append(machineName).append("\t\t\tArchitecture: ").append(machineArchitecture);
        if (!machineNotes.isEmpty()) {
            displayText.append("\nNotes: ").append(machineNotes);
        }
        machineTextView.setText(displayText.toString());
        machineTextView.setTextSize(18);
        return machineTextView;
    }


    private ImageButton createCloseButton(File machineFile, String machineName) {
        ImageButton closeButton = new ImageButton(getContext());
        LinearLayout.LayoutParams closeButtonParams = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
        );
        closeButton.setLayoutParams(closeButtonParams);
        closeButton.setImageResource(android.R.drawable.ic_menu_close_clear_cancel);
        closeButton.setBackgroundColor(Color.TRANSPARENT);
        closeButton.setPadding(8, 8, 8, 8);
        closeButton.setOnClickListener(v -> showDeleteConfirmationDialog(machineFile, machineName));
        return closeButton;
    }

    private void showErrorToast() {
        Toast.makeText(getContext(), "Error reading machine JSON", Toast.LENGTH_SHORT).show();
    }

    private void showDeleteConfirmationDialog(final File machineFile, String machineName) {
        AlertDialog.Builder builder = new AlertDialog.Builder(getContext());
        builder.setMessage("Are you sure you want to delete this machine: " + machineName + "?");
        builder.setPositiveButton("Yes", (dialog, which) -> deleteMachine(machineFile));
        builder.setNegativeButton("No", null);
        builder.create().show();
    }

    private void deleteMachine(File machineFile) {
        if (machineFile.exists()) {
            machineFile.delete();
        }
        restartActivity();
    }

    private void restartActivity() {
        Intent intent = getActivity().getIntent();
        getActivity().finish();
        startActivity(intent);
    }

    private void openMachineDetailsFragment(File machineFile) {
        try {
            FileInputStream inputStream = new FileInputStream(machineFile);
            int size = inputStream.available();
            byte[] buffer = new byte[size];
            inputStream.read(buffer);
            inputStream.close();
            String json = new String(buffer, "UTF-8");

            JSONObject machineJson = new JSONObject(json);
            String machineName = machineJson.optString("name");
            String machineArchitecture = machineJson.optString("architecture");
            String machineNotes = machineJson.optString("notes");
            String targetOS = machineJson.optString("os");
            String encryption = machineJson.optString("encryption");

            Bundle bundle = new Bundle();
            bundle.putString("machine_name", machineName);
            bundle.putString("machine_architecture", machineArchitecture);
            bundle.putString("machine_notes", machineNotes);
            bundle.putString("os", targetOS);
            bundle.putString("encryption", encryption);

            // Instead of creating a new instance of TargetFragment, navigate using the action
            NavDirections action = new NavHomeToNavTarget();
            Navigation.findNavController(requireView()).navigate(action.getActionId(), bundle);
        } catch (IOException | JSONException e) {
            e.printStackTrace();
            showErrorToast();
        }
    }

}

class NavHomeToNavTarget implements NavDirections {
    @Override
    public int getActionId() {
        return R.id.action_nav_home_to_nav_target;
    }

    @NonNull
    @Override
    public Bundle getArguments() {
        Bundle args = new Bundle();

        return args;
    }

    public static NavDirections actionNavHomeToNavTarget() {
        return new NavHomeToNavTarget();
    }
}