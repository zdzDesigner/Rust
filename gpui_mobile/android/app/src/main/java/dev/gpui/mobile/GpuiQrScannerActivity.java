package dev.gpui.mobile;

import android.app.Activity;
import android.os.Bundle;

import com.google.zxing.integration.android.IntentIntegrator;
import com.google.zxing.integration.android.IntentResult;

public final class GpuiQrScannerActivity extends Activity {
    private boolean delivered = false;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        if (savedInstanceState != null) {
            return;
        }

        IntentIntegrator integrator = new IntentIntegrator(this);
        integrator.setDesiredBarcodeFormats(IntentIntegrator.QR_CODE);
        integrator.setPrompt("请扫描二维码");
        integrator.setBeepEnabled(false);
        integrator.setOrientationLocked(false);
        integrator.initiateScan();
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, android.content.Intent data) {
        super.onActivityResult(requestCode, resultCode, data);

        IntentResult scanResult = IntentIntegrator.parseActivityResult(requestCode, resultCode, data);
        if (scanResult != null && scanResult.getContents() != null) {
            deliverResult(scanResult.getContents());
        } else {
            deliverResult(null);
        }
        finish();
    }

    @Override
    public void onBackPressed() {
        deliverResult(null);
        super.onBackPressed();
    }

    @Override
    protected void onDestroy() {
        deliverResult(null);
        super.onDestroy();
    }

    private void deliverResult(String value) {
        if (delivered) {
            return;
        }
        delivered = true;

        if (GpuiQrScanner.latch != null) {
            GpuiQrScanner.result.set(value);
            GpuiQrScanner.latch.countDown();
        }
    }
}
