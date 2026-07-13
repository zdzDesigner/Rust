package dev.gpui.mobile;

import android.app.Activity;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothManager;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.location.LocationManager;
import android.net.Uri;
import android.os.Build;
import android.provider.Settings;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicIntegerArray;

public final class GpuiPermissions {
    private static final int STATUS_GRANTED = 0;
    private static final int STATUS_DENIED = 1;
    private static final int STATUS_PERMANENTLY_DENIED = 2;

    public static int checkPermission(Activity activity, String permission) {
        if (permission == null || permission.isEmpty()) return STATUS_GRANTED;

        if ("android.permission.SYSTEM_ALERT_WINDOW".equals(permission)) {
            if (Build.VERSION.SDK_INT >= 23) {
                return Settings.canDrawOverlays(activity) ? STATUS_GRANTED : STATUS_DENIED;
            }
            return STATUS_GRANTED;
        }

        if ("android.permission.POST_NOTIFICATIONS".equals(permission) && Build.VERSION.SDK_INT < 33) {
            return STATUS_GRANTED;
        }

        int result = activity.checkSelfPermission(permission);
        return result == PackageManager.PERMISSION_GRANTED ? STATUS_GRANTED : STATUS_DENIED;
    }

    public static int requestPermission(Activity activity, String permission) {
        if (permission == null || permission.isEmpty()) return STATUS_GRANTED;

        if (checkPermission(activity, permission) == STATUS_GRANTED) {
            return STATUS_GRANTED;
        }

        if ("android.permission.SYSTEM_ALERT_WINDOW".equals(permission)) {
            if (Build.VERSION.SDK_INT >= 23) {
                Intent intent = new Intent(Settings.ACTION_MANAGE_OVERLAY_PERMISSION,
                        Uri.parse("package:" + activity.getPackageName()));
                activity.startActivity(intent);
                return Settings.canDrawOverlays(activity) ? STATUS_GRANTED : STATUS_DENIED;
            }
            return STATUS_GRANTED;
        }

        return requestViaActivity(activity, new String[]{permission})[0];
    }

    public static String requestPermissions(Activity activity, String permissions) {
        if (permissions == null || permissions.isEmpty()) return "";

        String[] items = permissions.split("\\|");
        int[] results = requestViaActivity(activity, items);

        StringBuilder builder = new StringBuilder();
        for (int i = 0; i < results.length; i++) {
            if (i > 0) builder.append("|");
            builder.append(results[i]);
        }
        return builder.toString();
    }

    public static boolean isServiceEnabled(Activity activity, int serviceType) {
        switch (serviceType) {
            case 0:
                LocationManager locationManager = (LocationManager) activity.getSystemService(Context.LOCATION_SERVICE);
                if (locationManager == null) return false;
                return locationManager.isProviderEnabled(LocationManager.GPS_PROVIDER)
                        || locationManager.isProviderEnabled(LocationManager.NETWORK_PROVIDER);
            case 1:
                BluetoothManager bluetoothManager = (BluetoothManager) activity.getSystemService(Context.BLUETOOTH_SERVICE);
                if (bluetoothManager == null) return false;
                BluetoothAdapter adapter = bluetoothManager.getAdapter();
                return adapter != null && adapter.isEnabled();
            default:
                return false;
        }
    }

    public static boolean openAppSettings(Activity activity) {
        try {
            Intent intent = new Intent(Settings.ACTION_APPLICATION_DETAILS_SETTINGS);
            intent.setData(Uri.parse("package:" + activity.getPackageName()));
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            activity.startActivity(intent);
            return true;
        } catch (Exception e) {
            android.util.Log.e("GpuiPermissions", "openAppSettings failed", e);
            return false;
        }
    }

    public static boolean shouldShowRationale(Activity activity, String permission) {
        if (permission == null || permission.isEmpty()) return false;
        return activity.shouldShowRequestPermissionRationale(permission);
    }

    private static int[] requestViaActivity(Activity activity, String[] permissions) {
        int[] finalResults = new int[permissions.length];
        boolean allGranted = true;
        for (int i = 0; i < permissions.length; i++) {
            if (checkPermission(activity, permissions[i]) == STATUS_GRANTED) {
                finalResults[i] = STATUS_GRANTED;
            } else {
                finalResults[i] = -1;
                allGranted = false;
            }
        }

        if (allGranted) return finalResults;

        java.util.List<String> needed = new java.util.ArrayList<>();
        java.util.List<Integer> neededIndices = new java.util.ArrayList<>();
        for (int i = 0; i < permissions.length; i++) {
            if (finalResults[i] == -1) {
                needed.add(permissions[i]);
                neededIndices.add(i);
            }
        }

        String[] neededArray = needed.toArray(new String[0]);
        CountDownLatch requestLatch = new CountDownLatch(1);
        GpuiPermissionActivity.latch = requestLatch;
        GpuiPermissionActivity.permissions = neededArray;
        GpuiPermissionActivity.results = new AtomicIntegerArray(neededArray.length);
        for (int i = 0; i < neededArray.length; i++) {
            GpuiPermissionActivity.results.set(i, PackageManager.PERMISSION_DENIED);
        }

        Intent intent = new Intent(activity, GpuiPermissionActivity.class);
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        activity.startActivity(intent);

        try {
            if (!requestLatch.await(60, TimeUnit.SECONDS)) {
                android.util.Log.w("GpuiPermissions", "permission request timed out");
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }

        for (int i = 0; i < neededIndices.size(); i++) {
            int index = neededIndices.get(i);
            int grantResult = GpuiPermissionActivity.results.get(i);
            if (grantResult == PackageManager.PERMISSION_GRANTED) {
                finalResults[index] = STATUS_GRANTED;
            } else if (!activity.shouldShowRequestPermissionRationale(neededArray[i])) {
                finalResults[index] = STATUS_PERMANENTLY_DENIED;
            } else {
                finalResults[index] = STATUS_DENIED;
            }
        }

        GpuiPermissionActivity.latch = null;
        GpuiPermissionActivity.permissions = null;
        GpuiPermissionActivity.results = null;
        return finalResults;
    }

    private GpuiPermissions() {}
}
