package dev.gpui.mobile;

import android.app.NativeActivity;
import android.content.pm.ActivityInfo;
import android.content.pm.PackageManager;
import android.os.Bundle;

import androidx.core.splashscreen.SplashScreen;

public final class GpuiActivity extends NativeActivity {
    private static volatile boolean nativeLibLoaded = false;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        SplashScreen splash = SplashScreen.installSplashScreen(this);
        loadNativeLibrary();
        splash.setKeepOnScreenCondition(() -> !isNativeReady());
        super.onCreate(savedInstanceState);
    }

    private void loadNativeLibrary() {
        if (nativeLibLoaded) {
            return;
        }

        try {
            ActivityInfo info = getPackageManager().getActivityInfo(
                    getComponentName(), PackageManager.GET_META_DATA);
            String libName = info.metaData.getString("android.app.lib_name");
            if (libName != null) {
                System.loadLibrary(libName);
                nativeLibLoaded = true;
            }
        } catch (PackageManager.NameNotFoundException ignored) {
            throw new IllegalStateException("GpuiActivity manifest meta-data missing for " + getComponentName(), ignored);
        } catch (UnsatisfiedLinkError ignored) {
            nativeLibLoaded = true;
        }
    }

    private boolean isNativeReady() {
        if (!nativeLibLoaded) {
            return false;
        }

        try {
            return nativeIsInitialized();
        } catch (UnsatisfiedLinkError ignored) {
            return false;
        }
    }

    private static native boolean nativeIsInitialized();
}
