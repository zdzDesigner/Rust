package dev.gpui.mobile;

import android.app.Activity;
import android.content.Intent;

public final class GpuiWebView {
    public static void openUrl(Activity activity, String url) {
        Intent intent = new Intent(activity, GpuiWebViewActivity.class);
        intent.putExtra(GpuiWebViewActivity.EXTRA_URL, url);
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        activity.startActivity(intent);
    }

    public static void openHtml(Activity activity, String html) {
        Intent intent = new Intent(activity, GpuiWebViewActivity.class);
        intent.putExtra(GpuiWebViewActivity.EXTRA_HTML, html);
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        activity.startActivity(intent);
    }

    private GpuiWebView() {}
}
