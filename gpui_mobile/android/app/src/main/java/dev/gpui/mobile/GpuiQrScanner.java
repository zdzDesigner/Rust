package dev.gpui.mobile;

import android.app.Activity;
import android.content.Intent;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicReference;

public final class GpuiQrScanner {
    private static final Object LOCK = new Object();
    static volatile CountDownLatch latch;
    static final AtomicReference<String> result = new AtomicReference<>(null);

    public static String scanQrCode(Activity activity) {
        CountDownLatch scanLatch = new CountDownLatch(1);

        synchronized (LOCK) {
            if (latch != null) {
                android.util.Log.w("GpuiQrScanner", "scan already in progress");
                return null;
            }
            latch = scanLatch;
            result.set(null);
        }

        Intent intent = new Intent(activity, GpuiQrScannerActivity.class);
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        activity.startActivity(intent);

        try {
            if (!scanLatch.await(120, TimeUnit.SECONDS)) {
                android.util.Log.w("GpuiQrScanner", "scan timed out");
                return null;
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return null;
        } finally {
            synchronized (LOCK) {
                latch = null;
            }
        }

        return result.get();
    }

    private GpuiQrScanner() {}
}
