package org.protea.os.launcher;

import android.app.Activity;
import android.content.Intent;
import android.content.SharedPreferences;
import android.content.pm.ApplicationInfo;
import android.content.pm.PackageManager;
import android.content.pm.ResolveInfo;
import android.os.Bundle;
import android.widget.Button;
import android.widget.TextView;
import android.widget.LinearLayout;

import java.util.List;

public final class MainActivity extends Activity {
    private static final String PREFS = "protea_launcher";
    private static final String MODE_KEY = "mode";
    private TextView mode;
    private SharedPreferences preferences;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        preferences = getSharedPreferences(PREFS, MODE_PRIVATE);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(48, 72, 48, 48);

        TextView title = new TextView(this);
        title.setText("PROTEA");
        title.setTextSize(34);
        title.setTypeface(null, 1);

        TextView subtitle = new TextView(this);
        subtitle.setText("One identity. One system. Adaptive everywhere.");
        subtitle.setTextSize(16);

        mode = new TextView(this);
        mode.setText("Mode: " + preferences.getString(MODE_KEY, "Office"));
        mode.setTextSize(18);

        LinearLayout modes = new LinearLayout(this);
        modes.setOrientation(LinearLayout.HORIZONTAL);

        Button gaming = new Button(this);
        gaming.setText("Gaming");
        gaming.setOnClickListener(v -> setMode("Gaming"));

        Button office = new Button(this);
        office.setText("Office");
        office.setOnClickListener(v -> setMode("Office"));

        modes.addView(gaming);
        modes.addView(office);

        TextView appsTitle = new TextView(this);
        appsTitle.setText("Applications");
        appsTitle.setTextSize(22);
        appsTitle.setPadding(0, 36, 0, 12);

        root.addView(title);
        root.addView(subtitle);
        root.addView(mode);
        root.addView(modes);
        root.addView(appsTitle);

        addLaunchableApplications(root);
        setContentView(root);
    }

    private void setMode(String value) {
        preferences.edit().putString(MODE_KEY, value).apply();
        mode.setText("Mode: " + value);
    }

    private void addLaunchableApplications(LinearLayout root) {
        Intent queryIntent = new Intent(Intent.ACTION_MAIN);
        queryIntent.addCategory(Intent.CATEGORY_LAUNCHER);

        PackageManager packageManager = getPackageManager();
        List<ResolveInfo> apps = packageManager.queryIntentActivities(queryIntent, 0);

        for (ResolveInfo info : apps) {
            ApplicationInfo appInfo = info.activityInfo.applicationInfo;
            String label = packageManager.getApplicationLabel(appInfo).toString();
            Button launch = new Button(this);
            launch.setText(label);
            launch.setOnClickListener(v -> {
                Intent launchIntent = packageManager.getLaunchIntentForPackage(appInfo.packageName);
                if (launchIntent != null) {
                    startActivity(launchIntent);
                }
            });
            root.addView(launch);
        }
    }
}
