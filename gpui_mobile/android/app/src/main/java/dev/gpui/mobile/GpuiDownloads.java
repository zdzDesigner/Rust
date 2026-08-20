package dev.gpui.mobile;

import android.app.Activity;
import android.content.ContentResolver;
import android.content.ContentValues;
import android.net.Uri;
import android.os.Build;
import android.os.Environment;
import android.provider.MediaStore;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * 把下载到 app 私有目录的文件保存到公共 Downloads（下载）目录。
 *
 * <p>targetSdk 34 下无法直接写 /storage/emulated/0/Download，
 * API 29+ 走 MediaStore.Downloads，API 26-28 走公共目录 + 存储权限。</p>
 *
 * <p>Called from Rust via JNI.</p>
 */
public final class GpuiDownloads {

    /**
     * @param activity  The current Activity.
     * @param cachePath 下载完成的缓存文件绝对路径（保存后会被删除）。
     * @param name      显示名称。
     * @return 保存位置的用户可读描述，失败返回 null。
     */
    public static String saveToDownloads(Activity activity, String cachePath, String name) {
        File src = new File(cachePath);
        try {
            String location;
            if (Build.VERSION.SDK_INT >= 29) {
                location = saveViaMediaStore(activity, src, name);
            } else {
                location = saveViaPublicDir(activity, src, name);
            }
            if (!src.delete()) {
                android.util.Log.w("GpuiDownloads", "无法删除缓存文件: " + cachePath);
            }
            return location;
        } catch (Exception e) {
            android.util.Log.e("GpuiDownloads", "saveToDownloads failed: " + name, e);
            return null;
        }
    }

    private static String saveViaMediaStore(Activity activity, File src, String name) throws Exception {
        ContentResolver resolver = activity.getContentResolver();

        ContentValues values = new ContentValues();
        values.put(MediaStore.Downloads.DISPLAY_NAME, name);
        values.put(MediaStore.Downloads.MIME_TYPE, "application/octet-stream");
        values.put(MediaStore.Downloads.IS_PENDING, 1);

        Uri uri = resolver.insert(MediaStore.Downloads.EXTERNAL_CONTENT_URI, values);
        if (uri == null) {
            throw new IOException("MediaStore insert 返回 null");
        }

        try (InputStream in = new FileInputStream(src);
             OutputStream out = resolver.openOutputStream(uri)) {
            if (out == null) {
                resolver.delete(uri, null, null);
                throw new IOException("MediaStore openOutputStream 返回 null");
            }
            copy(in, out);
        } catch (Exception e) {
            resolver.delete(uri, null, null);
            throw e;
        }

        values.clear();
        values.put(MediaStore.Downloads.IS_PENDING, 0);
        resolver.update(uri, values, null, null);

        return "下载/" + name;
    }

    private static String saveViaPublicDir(Activity activity, File src, String name) throws Exception {
        if (androidx.core.content.ContextCompat.checkSelfPermission(
                activity, "android.permission.WRITE_EXTERNAL_STORAGE")
                != android.content.pm.PackageManager.PERMISSION_GRANTED) {
            throw new IOException("缺少存储权限（WRITE_EXTERNAL_STORAGE）");
        }
        File dir = Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOWNLOADS);
        if (!dir.exists() && !dir.mkdirs()) {
            throw new IOException("创建下载目录失败: " + dir.getAbsolutePath());
        }
        File out = uniqueFile(dir, name);
        try (InputStream in = new FileInputStream(src);
             OutputStream os = new FileOutputStream(out)) {
            copy(in, os);
        }
        return out.getAbsolutePath();
    }

    private static void copy(InputStream in, OutputStream out) throws IOException {
        byte[] buf = new byte[65536];
        int n;
        while ((n = in.read(buf)) > 0) {
            out.write(buf, 0, n);
        }
    }

    private static File uniqueFile(File dir, String name) {
        File out = new File(dir, name);
        if (!out.exists()) {
            return out;
        }
        int dot = name.lastIndexOf('.');
        String base = dot > 0 ? name.substring(0, dot) : name;
        String ext = dot > 0 ? name.substring(dot) : "";
        for (int i = 1; ; i++) {
            out = new File(dir, base + "_" + i + ext);
            if (!out.exists()) {
                return out;
            }
        }
    }

    private GpuiDownloads() {}
}
