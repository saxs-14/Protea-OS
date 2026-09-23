package org.protea.os.launcher;

import android.app.Activity;
import android.os.Bundle;
import android.widget.Button;
import android.widget.TextView;
import android.widget.LinearLayout;

public final class MainActivity extends Activity {
    private TextView mode;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

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
        mode.setText("Mode: Office");
        mode.setTextSize(18);

        Button gaming = new Button(this);
        gaming.setText("Gaming");
        gaming.setOnClickListener(v -> mode.setText("Mode: Gaming"));

        Button office = new Button(this);
        office.setText("Office");
        office.setOnClickListener(v -> mode.setText("Mode: Office"));

        root.addView(title);
        root.addView(subtitle);
        root.addView(mode);
        root.addView(gaming);
        root.addView(office);

        setContentView(root);
    }
}
