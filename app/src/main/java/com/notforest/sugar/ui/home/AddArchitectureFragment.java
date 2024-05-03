package com.notforest.sugar.ui.home;

import android.content.Intent;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.CheckBox;
import android.widget.EditText;
import android.widget.Spinner;
import android.widget.Toast;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.fragment.app.DialogFragment;

import com.notforest.sugar.R;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

public class AddArchitectureFragment extends DialogFragment {

    private View root;
    private EditText editTextName, editTextNotes;
    private Spinner spinnerArchitecture, spinnerOS, spinnerEncryption;
    private CheckBox checkBoxErrorLog, checkBoxDebugLog, checkBoxTransactionLog;
    private Button addButton;

    @Nullable
    @Override
    public View onCreateView(@NonNull LayoutInflater inflater, @Nullable ViewGroup container, @Nullable Bundle savedInstanceState) {
        root = inflater.inflate(R.layout.view_add_architecture, container, false);

        editTextName = root.findViewById(R.id.editTextName);
        editTextNotes = root.findViewById(R.id.editTextNotes);
        spinnerArchitecture = root.findViewById(R.id.spinnerArchitecture);
        spinnerOS = root.findViewById(R.id.spinnerOS);
        spinnerEncryption = root.findViewById(R.id.spinnerEncryption);
        addButton = root.findViewById(R.id.add_arch_button);
        checkBoxErrorLog = root.findViewById(R.id.checkBoxErrorLog);
        checkBoxDebugLog = root.findViewById(R.id.checkBoxDebugLog);
        checkBoxTransactionLog = root.findViewById(R.id.checkBoxTransactionLog);


        addButton.setOnClickListener(v -> {
            // Add new target
            addNewTarget();
        });

        return root;
    }

    private void addNewTarget() {
        String machineName = editTextName.getText().toString();
        String architecture = spinnerArchitecture.getSelectedItem().toString();
        String os = spinnerOS.getSelectedItem().toString();
        String encryption = spinnerEncryption.getSelectedItem().toString();
        String notes = editTextNotes.getText().toString();

        if (machineName.isEmpty()) {
            Toast.makeText(getContext(), "Machine name cannot be empty", Toast.LENGTH_SHORT).show();
            return;
        }

        JSONObject machineJson = new JSONObject();
        try {
            machineJson.put("name", machineName);
            machineJson.put("architecture", architecture);
            machineJson.put("os", os);
            machineJson.put("encryption", encryption);
            machineJson.put("notes", notes);
            machineJson.put("errorLog", checkBoxErrorLog.isChecked());
            machineJson.put("debugLog", checkBoxDebugLog.isChecked());
            machineJson.put("transactionLog", checkBoxTransactionLog.isChecked());
        } catch (JSONException e) {
            e.printStackTrace();
            Toast.makeText(getContext(), "Error creating JSON", Toast.LENGTH_SHORT).show();
            return;
        }

        saveMachine(machineJson, architecture, machineName);

        Toast.makeText(getContext(), "Added " + machineName + " successfully.", Toast.LENGTH_SHORT).show();

        dismiss();

        Intent intent = getActivity().getIntent();
        getActivity().finish();
        startActivity(intent);
    }


    private void saveMachine(JSONObject machineJson, String architecture, String machineName) {
        File architectureDir = new File(getContext().getFilesDir(), architecture);
        if (!architectureDir.exists()) {
            architectureDir.mkdirs();
        }

        File machineFile = new File(architectureDir, machineName + ".json");
        try {
            FileOutputStream outputStream = new FileOutputStream(machineFile);
            outputStream.write(machineJson.toString().getBytes());
            outputStream.close();
        } catch (IOException e) {
            e.printStackTrace();
            Toast.makeText(getContext(), "Error saving machine data", Toast.LENGTH_SHORT).show();
        }
    }
}
