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

import android.app.PendingIntent;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.content.SharedPreferences;
import android.graphics.Color;
import android.graphics.drawable.Drawable;
import android.graphics.drawable.LayerDrawable;
import android.hardware.usb.UsbDevice;
import android.hardware.usb.UsbDeviceConnection;
import android.hardware.usb.UsbManager;
import android.os.Bundle;
import android.text.Spannable;
import android.text.SpannableString;
import android.text.SpannableStringBuilder;
import android.text.style.ForegroundColorSpan;
import android.util.Log;
import android.view.Gravity;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.view.inputmethod.EditorInfo;
import android.widget.Button;
import android.widget.EditText;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.TextView;
import android.widget.ViewFlipper;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.core.content.ContextCompat;
import androidx.fragment.app.Fragment;

import com.notforest.sugar.MainActivity;
import com.notforest.sugar.R;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

public class TargetFragment extends Fragment {

    // Loads external dynamic libraries.
    static {
        System.loadLibrary("sugar_jni");
    }

    // Native JNI interface for Rust backend.
    private static native int connect(int fd);
    private static native int disconnect();
    private static native String conn_info();

    private static final String ACTION_USB_PERMISSION = "com.notforest.sugar.USB_PERMISSION";

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
    private HashMap<String, UsbDevice> deviceList;
    private UsbManager usbManager;
    private LinearLayout diskContentLayout;

    private final BroadcastReceiver usbPermissionReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            String action = intent.getAction();
            if (ACTION_USB_PERMISSION.equals(action)) {
                synchronized (this) {
                    UsbDevice device = intent.getParcelableExtra(UsbManager.EXTRA_DEVICE);
                    if (intent.getBooleanExtra(UsbManager.EXTRA_PERMISSION_GRANTED, false)) {
                        if (device != null) {
                            UsbDeviceConnection usbDeviceConnection = usbManager.openDevice(device);
                            int fileDescriptor = usbDeviceConnection.getFileDescriptor();
                            switch (connect(fileDescriptor)) {
                                case 0:
                                    POWER = true;
                                    sharedPreferences.edit().putBoolean("power_" + machineNameTextView.getText(), false).apply();
                                    displayMessage("info:" + getString(R.string.connected_to_device) + conn_info());
                                    break;
                                default:
                                    displayMessage("error: " + getString(R.string.error_unknown_error));
                            }
                        }
                    } else {
                        Log.d("USB Permission", "Permission denied for device " + device);
                        displayMessage("error: " + getString(R.string.error_usb_permission_denied));
                    }
                }
            }
        }
    };

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
        diskContentLayout = root.findViewById(R.id.disk_content);

        MainActivity mainActivity = (MainActivity) getActivity();
        usbManager = (UsbManager) mainActivity.getSystemService(Context.USB_SERVICE);

        PendingIntent permissionIntent = PendingIntent.getBroadcast(requireContext(), 0, new Intent(ACTION_USB_PERMISSION), PendingIntent.FLAG_IMMUTABLE);
        deviceList = usbManager.getDeviceList();
        for (UsbDevice usbDevice : deviceList.values()) {
            usbManager.requestPermission(usbDevice, permissionIntent);
        }

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
            UsbDevice chosenDevice = deviceList.isEmpty() ? null : deviceList.values().iterator().next();

            for (UsbDevice usbDevice : deviceList.values()) {
                usbManager.requestPermission(usbDevice, permissionIntent);
            }

            try_connect(chosenDevice);
        });

        ViewFlipper viewFlipper = root.findViewById(R.id.view_flipper);
        Button toggleViewButton = root.findViewById(R.id.bookmark_button);
        TextView headerTextView = root.findViewById(R.id.terminal_title);

        toggleViewButton.setOnClickListener(v -> {
            viewFlipper.showNext();
            // Update the text of the header based on the current view
            if (viewFlipper.getDisplayedChild() == 0) {
                headerTextView.setText(getString(R.string.storage));
            } else {
                headerTextView.setText(getString(R.string.terminal));
            }
        });

        return root;
    }

    @Override
    public void onResume() {
        super.onResume();
        IntentFilter filter = new IntentFilter(ACTION_USB_PERMISSION);
        requireActivity().registerReceiver(usbPermissionReceiver, filter);
    }

    @Override
    public void onPause() {
        super.onPause();
        requireActivity().unregisterReceiver(usbPermissionReceiver);
    }

    private void loadMessageBuffer() {
        // Load message buffer from SharedPreferences
        for (int i = 0; i < 100; i++) {
            String message = sharedPreferences.getString(machineNameTextView.getText() + "message_" + i, null);
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
            if (message.toLowerCase().startsWith(" > info:")) {
                spannableString.setSpan(new ForegroundColorSpan(Color.YELLOW), 0, spannableString.length(), Spannable.SPAN_EXCLUSIVE_EXCLUSIVE);
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

    private void try_connect(final UsbDevice chosenDevice) {
        if (chosenDevice == null) {
            displayMessage("error: " + getString(R.string.error_device_null));
        } else {
            Thread connectThread = new Thread(() -> {
                if (usbManager.hasPermission(chosenDevice)) {
                    UsbDeviceConnection usbDeviceConnection = usbManager.openDevice(chosenDevice);
                    int fileDescriptor = usbDeviceConnection.getFileDescriptor();
                    if (!POWER) {
                        displayMessage(getString(R.string.connecting_to_device) + chosenDevice.getDeviceName());
                        displayMessage("info: " + getString(R.string.flashing_the_daemon));
                        switch (connect(fileDescriptor)) {
                            case 0:
                                POWER = !POWER;
                                sharedPreferences.edit().putBoolean("power_" + machineNameTextView.getText(), POWER).apply();
                                displayMessage("info: " + getString(R.string.connected_to_device) + conn_info());
                                break;
                            default:
                                displayMessage("error: " + getString(R.string.error_unknown_error));
                        }
                    } else {
                        switch (disconnect()) {
                            case 0:
                                POWER = !POWER;
                                sharedPreferences.edit().putBoolean("power_" + machineNameTextView.getText(), POWER).apply();
                                break;
                            default:
                                displayMessage("error: " + getString(R.string.error_unknown_error));
                        }
                    }
                } else {
                    displayMessage("error: " + getString(R.string.error_usb_permission_denied));
                }
            });

            connectThread.start();
        }
    }


    /* Parses all commands from the user's side. */
    private void parseCommand(String command) {
        switch (command) {
            case "connect":
                if (!POWER) {
                    UsbDevice chosenDevice = deviceList.isEmpty() ? null : deviceList.values().iterator().next();
                    try_connect(chosenDevice);
                } else {
                    displayMessage("error: " + getString(R.string.error_device_connected));
                }
            case "disconnect":
                if (POWER) {
                    UsbDevice chosenDevice = deviceList.isEmpty() ? null : deviceList.values().iterator().next();
                    try_connect(chosenDevice);
                } else {
                    displayMessage("error: " + getString(R.string.error_device_not_connected));
                }
                break;
            case "help":
                displayMessage("info: " + getString(R.string.cmd_help_info));
                break;
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

    private void populateDiskContent(List<FileStructure.Disk> disks) {
        if (disks != null && !disks.isEmpty()) {
            // Removing "nothing yet" view if present
            View diskEmptyStateView = root.findViewById(R.id.disk_empty_state);
            if (diskEmptyStateView != null) {
                ((ViewGroup) diskEmptyStateView.getParent()).removeView(diskEmptyStateView);
            }

            // Add the disks to the layout
            for (FileStructure.Disk disk : disks) {
                LinearLayout diskLayout = createLinearLayoutWithIcon(disk.name, R.drawable.baseline_insert_drive_file_24, 24, 1);
                diskContentLayout.addView(diskLayout);

                LinearLayout partitionLayout = new LinearLayout(requireContext());
                partitionLayout.setOrientation(LinearLayout.VERTICAL);
                partitionLayout.setVisibility(View.GONE);
                diskContentLayout.addView(partitionLayout);

                diskLayout.setOnClickListener(v -> toggleVisibility(partitionLayout));

                for (FileStructure.Partition partition : disk.partitions) {
                    LinearLayout partitionView = createLinearLayoutWithIcon(partition.name, R.drawable.baseline_repartition_24, 22, 4);
                    partitionLayout.addView(partitionView);

                    LinearLayout directoryLayout = new LinearLayout(requireContext());
                    directoryLayout.setOrientation(LinearLayout.VERTICAL);
                    directoryLayout.setVisibility(View.GONE);
                    partitionLayout.addView(directoryLayout);

                    partitionView.setOnClickListener(v -> toggleVisibility(directoryLayout));

                    for (FileStructure.Directory directory : partition.directories) {
                        addDirectoryView(directory, directoryLayout, 20, 8);
                    }
                }
            }
        }
    }

    private void addDirectoryView(FileStructure.Directory directory, LinearLayout parentLayout, int textSize, int indentationLevel) {
        LinearLayout directoryView = createLinearLayoutWithIcon(directory.name, R.drawable.baseline_folder_24, textSize, indentationLevel);
        parentLayout.addView(directoryView);

        LinearLayout subdirectoryLayout = new LinearLayout(requireContext());
        subdirectoryLayout.setOrientation(LinearLayout.VERTICAL);
        subdirectoryLayout.setVisibility(View.GONE);
        parentLayout.addView(subdirectoryLayout);

        directoryView.setOnClickListener(v -> toggleVisibility(subdirectoryLayout));

        if (directory.directories != null) {
            for (FileStructure.Directory subdirectory : directory.directories) {
                addDirectoryView(subdirectory, subdirectoryLayout, textSize - 2, indentationLevel + 2);
            }
        }

        if (directory.files != null) {
            for (String file : directory.files) {
                String fileType = getFileType(file);
                int iconResId = getIconForFileType(fileType);
                TextView fileView = createTextView(file, textSize - 2, indentationLevel + 1, iconResId);
                parentLayout.addView(fileView);
            }
        }
    }

    private String getFileType(String filename) {
        String[] parts = filename.split("\\.");
        if (parts.length > 1) {
            return parts[parts.length - 1];
        } else {
            return "";
        }
    }

    private int getIconForFileType(String fileType) {
        switch (fileType.toLowerCase()) {
            case "mp3":
            case "wav":
            case "flac":
                return R.drawable.baseline_audio_file_24;
            case "js":
                return R.drawable.baseline_javascript_24;
            case "txt":
                return R.drawable.baseline_short_text_24;
            case "exe":
                return R.drawable.baseline_reply_24;
            default:
                return R.drawable.baseline_folder_24;
        }
    }

    private TextView createTextView(String text, int textSize, int indentationLevel, int iconResId) {
        TextView textView = new TextView(requireContext());
        LinearLayout.LayoutParams layoutParams = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT);
        layoutParams.setMargins(16 * indentationLevel, 0, 0, 0);
        textView.setLayoutParams(layoutParams);
        textView.setText(text);
        textView.setTextSize(textSize);
        textView.setPadding(16, 8, 16, 8);

        // Set the icon to the left of the text
        Drawable icon = ContextCompat.getDrawable(requireContext(), iconResId);
        if (icon != null) {
            icon.setBounds(0, 0, icon.getIntrinsicWidth(), icon.getIntrinsicHeight());
            textView.setCompoundDrawables(icon, null, null, null);
        }

        return textView;
    }


    private LinearLayout createLinearLayoutWithIcon(String text, int iconResId, int textSize, int indentationLevel) {
        LinearLayout linearLayout = new LinearLayout(requireContext());
        linearLayout.setOrientation(LinearLayout.HORIZONTAL);
        linearLayout.setPadding(16 * indentationLevel, 8, 16, 8);
        linearLayout.setGravity(Gravity.CENTER_VERTICAL);

        ImageView icon = new ImageView(requireContext());
        icon.setImageResource(iconResId);
        LinearLayout.LayoutParams iconLayoutParams = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT,
                LinearLayout.LayoutParams.WRAP_CONTENT);
        iconLayoutParams.setMarginEnd(16);
        icon.setLayoutParams(iconLayoutParams);
        linearLayout.addView(icon);

        TextView textView = new TextView(requireContext());
        textView.setText(text);
        textView.setTextSize(textSize);
        textView.setLayoutParams(new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT));
        linearLayout.addView(textView);

        return linearLayout;
    }

    private void toggleVisibility(View view) {
        view.setVisibility(view.getVisibility() == View.VISIBLE ? View.GONE : View.VISIBLE);
    }

}

class FileStructure {
    public static class Disk {
        String name;
        List<Partition> partitions;

        Disk(String name, List<Partition> partitions) {
            this.name = name;
            this.partitions = partitions;
        }
    }

    public static class Partition {
        String name;
        List<Directory> directories;

        Partition(String name, List<Directory> directories) {
            this.name = name;
            this.directories = directories;
        }
    }

    public static class Directory {
        String name;
        List<Directory> directories;
        List<String> files;

        Directory(String name, List<Directory> directories, List<String> files) {
            this.name = name;
            this.directories = directories != null ? directories : new ArrayList<>();
            this.files = files != null ? files : new ArrayList<>();
        }
    }
}

