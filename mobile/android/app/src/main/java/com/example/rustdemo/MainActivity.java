package com.example.rustdemo;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;
import android.widget.LinearLayout;
import android.view.Gravity;
import android.util.TypedValue;

public class MainActivity extends Activity {
    
    static {
        // 加载 Rust 编译的共享库
        System.loadLibrary("rust_android_demo");
    }
    
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        // 创建布局
        LinearLayout layout = new LinearLayout(this);
        layout.setOrientation(LinearLayout.VERTICAL);
        layout.setGravity(Gravity.CENTER);
        layout.setPadding(50, 50, 50, 50);
        
        // 调用 Rust 函数获取字符串
        String rustString = RustLib.stringFromJNI();
        
        // 创建文本视图
        TextView textView = new TextView(this);
        textView.setText(rustString);
        textView.setTextSize(TypedValue.COMPLEX_UNIT_SP, 24);
        textView.setGravity(Gravity.CENTER);
        
        // 调用 Rust 加法函数
        int result = RustLib.addFromJNI(42, 58);
        TextView resultView = new TextView(this);
        resultView.setText("42 + 58 = " + result);
        resultView.setTextSize(TypedValue.COMPLEX_UNIT_SP, 20);
        resultView.setGravity(Gravity.CENTER);
        resultView.setPadding(0, 30, 0, 0);
        
        layout.addView(textView);
        layout.addView(resultView);
        
        setContentView(layout);
    }
}
