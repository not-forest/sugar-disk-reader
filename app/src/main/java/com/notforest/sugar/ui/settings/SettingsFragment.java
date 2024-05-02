package com.notforest.sugar.ui.settings;

import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.content.res.Configuration;
import android.content.res.Resources;
import android.content.res.TypedArray;
import android.graphics.Color;
import android.os.Bundle;
import android.util.DisplayMetrics;
import android.util.TypedValue;
import android.view.Gravity;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.AdapterView;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.PopupWindow;
import android.widget.SeekBar;
import android.widget.Spinner;
import android.widget.Toast;
import android.widget.Toolbar;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;
import androidx.localbroadcastmanager.content.LocalBroadcastManager;

import com.notforest.sugar.MainActivity;
import com.notforest.sugar.R;

import java.util.Locale;

public class SettingsFragment extends Fragment {

    private Button mColorPickerButton;
    private View root;
    private int red, green, blue;
    private Context mContext;

    @Override
    public void onAttach(@NonNull Context context) {
        super.onAttach(context);
        mContext = context; // Assign the context when fragment is attached
    }

    public View onCreateView(@NonNull LayoutInflater inflater,
                             ViewGroup container, Bundle savedInstanceState) {
        root = inflater.inflate(R.layout.fragment_settings, container, false);
        MainActivity mainActivity = (MainActivity) getActivity();
        SharedPreferences prefs = mainActivity.getSharedPreferences("AppPrefs", Context.MODE_PRIVATE);

        mColorPickerButton = root.findViewById(R.id.color_picker_button);
        mColorPickerButton.setOnClickListener(v -> showColorPicker(prefs, mainActivity));

        Spinner languageSpinner = root.findViewById(R.id.language_spinner);
        ArrayAdapter<CharSequence> adapter = ArrayAdapter.createFromResource(mContext,
                R.array.languages_array, android.R.layout.simple_spinner_item);
        adapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item);
        languageSpinner.setAdapter(adapter);

        String currentLang = getResources().getConfiguration().locale.getDisplayLanguage();
        languageSpinner.setSelection(adapter.getPosition(currentLang));

        languageSpinner.setOnItemSelectedListener(new AdapterView.OnItemSelectedListener() {
            @Override
            public void onItemSelected(AdapterView<?> parentView, View selectedItemView, int position, long id) {
                String selectedLanguage = parentView.getItemAtPosition(position).toString();
                Locale newLocale;
                switch (selectedLanguage) {
                    case "Polish":
                        newLocale = new Locale("pl");
                        break;
                    case "Ukrainian":
                        newLocale = new Locale("uk");
                        break;
                    default:
                        newLocale = Locale.ENGLISH;
                }

                Locale currentLocale = getResources().getConfiguration().locale;
                if (!newLocale.equals(currentLocale)) {
                    Resources resources = getResources();
                    Configuration config = resources.getConfiguration();
                    DisplayMetrics metrics = resources.getDisplayMetrics();
                    config.setLocale(newLocale);
                    resources.updateConfiguration(config, metrics);
                    prefs.edit().putString("selectedLocale", newLocale.toString()).apply();
                    mainActivity.recreate();
                }
            }

            @Override
            public void onNothingSelected(AdapterView<?> parentView) {
                // Do nothing
            }
        });


        return root;
    }

    private void showColorPicker(SharedPreferences prefs, MainActivity mainActivity) {
        View popupView = LayoutInflater.from(getContext()).inflate(R.layout.color_picker_popup, null);
        SeekBar seekBarRed = popupView.findViewById(R.id.seekBarRed);
        SeekBar seekBarGreen = popupView.findViewById(R.id.seekBarGreen);
        SeekBar seekBarBlue = popupView.findViewById(R.id.seekBarBlue);
        View colorPreview = popupView.findViewById(R.id.preview_selected_color);

        // Set up listeners for the SeekBars
        SeekBar.OnSeekBarChangeListener listener = new SeekBar.OnSeekBarChangeListener() {
            @Override
            public void onProgressChanged(SeekBar seekBar, int progress, boolean fromUser) {
                if (seekBar == seekBarRed) {
                    red = progress;
                } else if (seekBar == seekBarGreen) {
                    green = progress;
                } else if (seekBar == seekBarBlue) {
                    blue = progress;
                }
                int color = Color.rgb(red, green, blue);
                colorPreview.setBackgroundColor(color);
            }

            @Override
            public void onStartTrackingTouch(SeekBar seekBar) {
            }

            @Override
            public void onStopTrackingTouch(SeekBar seekBar) {
            }
        };

        seekBarRed.setOnSeekBarChangeListener(listener);
        seekBarGreen.setOnSeekBarChangeListener(listener);
        seekBarBlue.setOnSeekBarChangeListener(listener);

        Button setColorButton = popupView.findViewById(R.id.set_color_button);
        setColorButton.setOnClickListener(v -> {
            int color = Color.rgb(red, green, blue);
            prefs.edit().putInt("selectedColor", color).apply();
            Toast.makeText(mainActivity, "Color saved successfully.", Toast.LENGTH_SHORT).show();
            createCustomTheme(color);
        });

        PopupWindow popupWindow = new PopupWindow(popupView, LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT, true);
        popupWindow.showAtLocation(root, Gravity.CENTER, 0, 0);
    }

    private void createCustomTheme(int color) {
        MainActivity mainActivity = (MainActivity) getActivity();
        androidx.appcompat.widget.Toolbar toolbar = mainActivity.findViewById(R.id.toolbar); // Replace toolbar with your actual toolbar id

        // Set the background color of the toolbar
        toolbar.setBackgroundColor(color);
        mainActivity.getWindow().setStatusBarColor(color);
    }

    @Override
    public void onDestroyView() {
        super.onDestroyView();
        root = null;
    }
}
