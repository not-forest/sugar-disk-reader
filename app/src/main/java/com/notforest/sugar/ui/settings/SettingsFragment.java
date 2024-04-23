package com.notforest.sugar.ui.settings;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;
import androidx.lifecycle.ViewModelProvider;

import com.notforest.sugar.MainActivity;
import com.notforest.sugar.databinding.FragmentHomeBinding;
import com.notforest.sugar.databinding.FragmentSettingsBinding;
import com.notforest.sugar.ui.home.HomeViewModel;


public class SettingsFragment extends Fragment {

    private FragmentSettingsBinding binding;
    private View root;

    public View onCreateView(@NonNull LayoutInflater inflater,
                             ViewGroup container, Bundle savedInstanceState) {
        HomeViewModel homeViewModel =
                new ViewModelProvider(this).get(HomeViewModel.class);

        binding = FragmentSettingsBinding.inflate(inflater, container, false);
        root = binding.getRoot();

        // Access the activity associated with the fragment
        MainActivity mainActivity = (MainActivity) getActivity();
        if (mainActivity != null) {
            int randomIndex = (int) (Math.random() * mainActivity.backgroundDrawables.length);
            root.setBackgroundResource(mainActivity.backgroundDrawables[randomIndex]);
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