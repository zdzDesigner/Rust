use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

#[no_mangle]
pub extern "C" fn Java_com_example_rustdemo_RustLib_stringFromJNI(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    // 初始化日志（输出到 logcat）
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    
    log::info!("Rust JNI function called from Android!");
    
    // 创建 Java 字符串并返回
    let output = env.new_string("Hello from Rust! 🦀").expect("Failed to create string");
    output.into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_example_rustdemo_RustLib_addFromJNI(
    _env: JNIEnv,
    _class: JClass,
    a: i32,
    b: i32,
) -> i32 {
    log::info!("Adding {} + {}", a, b);
    a + b
}
