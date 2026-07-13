package com.example.rustdemo;

public class RustLib {
    // 声明 native 方法，由 Rust 实现
    public static native String stringFromJNI();
    public static native int addFromJNI(int a, int b);
}
