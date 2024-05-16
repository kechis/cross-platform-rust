package com.mozilla.greetings;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.widget.TextView;

import com.test.RootUtils;

public class GreetingsActivity extends AppCompatActivity {

    static {
        System.loadLibrary("greetings");
        System.loadLibrary("root");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_greetings);

        RustGreetings g = new RustGreetings();
        String r = g.sayHello("world");
        ((TextView)findViewById(R.id.greetingField)).setText(r);

        RootUtils ru = new RootUtils();
        Log.d(getClass().getName(), String.format("method1 = %d", ru.method1()));
        Log.d(getClass().getName(), String.format("method2 = %d", ru.method2()));
        Log.d(getClass().getName(), String.format("method3 = %d", ru.method3()));
    }
}
