/*
 *  Main fragment for I/O communication with daemon on the selected device.
 *
 *  This fragment defines an interface to communicate with the daemon as well as uploading the daemon itself
 *  to the target architecture. All errors and debug messages can be seen at the terminal within this fragment,
 *  however certain settings may change this behaviour and messages could also appear on the screen of the target
 *  device.
 *
 *  The low level communication is happening on the backend side. Two devices must see each other via physical connection
 *  in order to transfer information.
 * */

package com.notforest.sugar.ui.home;

import android.content.Context;
import android.content.SharedPreferences;
import android.graphics.Color;
import android.graphics.drawable.Drawable;
import android.graphics.drawable.LayerDrawable;
import android.os.Bundle;
import android.text.Spannable;
import android.text.SpannableString;
import android.text.SpannableStringBuilder;
import android.text.style.ForegroundColorSpan;
import android.util.Log;
import android.view.KeyEvent;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.view.inputmethod.EditorInfo;
import android.widget.EditText;
import android.widget.ImageView;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.core.content.ContextCompat;
import androidx.fragment.app.Fragment;

import com.notforest.sugar.MainActivity;
import com.notforest.sugar.R;

import java.util.ArrayList;
import java.util.List;

public class TargetFragment extends Fragment {

    private View root;
    private TextView machineNameTextView,
            machineArchitectureTextView,
            machineOSTextView,
            machineNotesTextView,
            machineEncryptionTextView,
            terminalOutputTextView;
    private EditText terminalInputEditText;

    private List<String> messageBuffer;
    private SharedPreferences sharedPreferences;
    private ImageView powerButton;
    private Drawable buttonBackground;
    private boolean POWER;
    private static final String SHARED_PREFS_NAME = "MessageBuffer";

    @Nullable
    @Override
    public View onCreateView(@NonNull LayoutInflater inflater, @Nullable ViewGroup container, @Nullable Bundle savedInstanceState) {
        root = inflater.inflate(R.layout.fragment_target_menu, container, false);
        machineNameTextView = root.findViewById(R.id.machine_name);
        machineArchitectureTextView = root.findViewById(R.id.machine_architecture);
        machineNotesTextView = root.findViewById(R.id.machine_notes);
        machineOSTextView = root.findViewById(R.id.machine_os);
        machineEncryptionTextView = root.findViewById(R.id.machine_encryption);
        terminalOutputTextView = root.findViewById(R.id.terminal_output);
        terminalInputEditText = root.findViewById(R.id.terminal_input);
        powerButton = root.findViewById(R.id.power_button);
        buttonBackground = powerButton.getDrawable();

        MainActivity mainActivity = (MainActivity) getActivity();
        if (mainActivity != null) {
            int randomIndex = (int) (Math.random() * mainActivity.backgroundDrawables.length);
            root.setBackgroundResource(mainActivity.backgroundDrawables[randomIndex]);
        }

        messageBuffer = new ArrayList<>();
        sharedPreferences = requireActivity().getSharedPreferences(SHARED_PREFS_NAME, Context.MODE_PRIVATE);
        POWER = sharedPreferences.getBoolean("power_" + machineNameTextView.getText(), false);

        if (POWER) {
            if (buttonBackground instanceof LayerDrawable) {
                LayerDrawable layers = (LayerDrawable) buttonBackground;
                // Assuming the second layer is at index 1
                Drawable secondLayer = layers.getDrawable(1);
                if (secondLayer != null) {
                    secondLayer.setTint(ContextCompat.getColor(requireContext(), R.color.main_triadic));
                }
            }
        }

        Bundle bundle = getArguments();
        if (bundle != null) {
            String machineName = bundle.getString("machine_name");
            String machineArchitecture = bundle.getString("machine_architecture");
            String machineNotes = bundle.getString("machine_notes");
            String machineOS = bundle.getString("os");
            String machineEncryption = bundle.getString("encryption");

            machineNameTextView.setText(machineName);
            machineArchitectureTextView.setText(machineArchitecture);
            machineNotesTextView.setText(machineNotes);
            machineOSTextView.setText(machineOS);
            machineEncryptionTextView.setText(machineEncryption);
        }

        loadMessageBuffer();
        displayMessageBuffer();

        terminalInputEditText.setOnEditorActionListener((textView, actionId, keyEvent) -> {
            String userInput = terminalInputEditText.getText().toString().trim();
            if (actionId == EditorInfo.IME_ACTION_SEND) {
                if (!userInput.isEmpty()) {
                    displayMessage(userInput);
                    parseCommand(userInput);
                    terminalInputEditText.setText("");
                }
                return true;
            }
            Log.d("TargetFragment", "User input: " + userInput);
            return false;
        });

        powerButton.setOnClickListener(v -> {
            Log.d("POWER", POWER ? "ON" : "OFF");
            // TODO!!!
        });

        return root;
    }

    private void loadMessageBuffer() {
        // Load message buffer from SharedPreferences
        for (int i = 0; i < 100; i++) {
            String message = sharedPreferences.getString( machineNameTextView.getText() + "message_" + i, null);
            if (message != null) {
                messageBuffer.add(message);
            }
        }
    }

    private void displayMessageBuffer() {
        // Display messages in terminal output TextView
        SpannableStringBuilder builder = new SpannableStringBuilder();
        for (String message : messageBuffer) {
            SpannableString spannableString = new SpannableString(message + "\n");
            if (message.toLowerCase().startsWith(" > error:")) {
                spannableString.setSpan(new ForegroundColorSpan(Color.RED), 0, spannableString.length(), Spannable.SPAN_EXCLUSIVE_EXCLUSIVE);
            }
            builder.append(spannableString);
        }
        terminalOutputTextView.setText(builder);
    }

    private void appendMessageToBuffer(String message) {
        messageBuffer.add(" > " + message);
        SharedPreferences.Editor editor = sharedPreferences.edit();
        for (int i = 0; i < messageBuffer.size(); i++) {
            editor.putString(machineNameTextView.getText() + "message_" + i, messageBuffer.get(i));
        }
        editor.apply();
    }

    private void displayMessage(String message) {
        appendMessageToBuffer(message);
        displayMessageBuffer();
    }

    // Parsing user commands.
    private void parseCommand(String command) {
        switch (command) {
            case "clear":
                messageBuffer.clear();
                SharedPreferences.Editor editor = sharedPreferences.edit();
                for (int i = 0; i < 100; i++) {
                    editor.remove(machineNameTextView.getText() + "message_" + i);
                }
                editor.apply();
                displayMessageBuffer();
                break;
            default:
                displayMessage("error: " + getString(R.string.error_unknown_command_1) + command + getString(R.string.error_unknown_command_2));
        }
    }
}
