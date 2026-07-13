package dev.gpui.mobile;

import android.app.Activity;
import android.os.Bundle;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicIntegerArray;

public class GpuiPermissionActivity extends Activity {
    private static final int PERMISSION_REQUEST_CODE = 9002;
    private static final String KEY_WAITING = "gpui_waiting_for_permission";

    static CountDownLatch latch;
    static String[] permissions;
    static AtomicIntegerArray results;

    private boolean waitingForResult = false;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        if (savedInstanceState != null && savedInstanceState.getBoolean(KEY_WAITING, false)) {
            waitingForResult = true;
            return;
        }

        if (permissions != null && permissions.length > 0) {
            waitingForResult = true;
            requestPermissions(permissions, PERMISSION_REQUEST_CODE);
        } else {
            deliverResult();
            finish();
        }
    }

    @Override
    protected void onSaveInstanceState(Bundle outState) {
        super.onSaveInstanceState(outState);
        outState.putBoolean(KEY_WAITING, waitingForResult);
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults);
        waitingForResult = false;

        if (requestCode == PERMISSION_REQUEST_CODE && results != null) {
            for (int i = 0; i < grantResults.length && i < results.length(); i++) {
                results.set(i, grantResults[i]);
            }
        }

        deliverResult();
        finish();
    }

    @Override
    public void onBackPressed() {
        waitingForResult = false;
        deliverResult();
        super.onBackPressed();
    }

    private void deliverResult() {
        if (latch != null) {
            latch.countDown();
        }
    }
}
