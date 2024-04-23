/*
 *  Main Home fragment.
 *
 *  Handles all front-end IO communication with the user, when it comes to creating and managing
 *  targets for future reading. This fragment will also allow for real time communication with
 *  chosen architecture and provide parsed results from reading files.
 * */

package com.notforest.sugar.ui.home;

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

public class HomeFragment extends Fragment {

    private FragmentHomeBinding binding;
    private View root;

    public View onCreateView(@NonNull LayoutInflater inflater,
                             ViewGroup container, Bundle savedInstanceState) {
        HomeViewModel homeViewModel =
                new ViewModelProvider(this).get(HomeViewModel.class);

        binding = FragmentHomeBinding.inflate(inflater, container, false);
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