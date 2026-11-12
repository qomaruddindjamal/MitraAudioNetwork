Add-Type @"
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;

public class WindowSnapshot {
    [DllImport("user32.dll")]
    public static extern bool PrintWindow(IntPtr hWnd, IntPtr hdcBkgnd, uint nFlags);

    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);

    [StructLayout(LayoutKind.Sequential)]
    public struct RECT {
        public int Left;
        public int Top;
        public int Right;
        public int Bottom;
    }

    public static void CaptureWindow(IntPtr hWnd, string outputPath) {
        RECT rect;
        GetWindowRect(hWnd, out rect);
        int width = rect.Right - rect.Left;
        int height = rect.Bottom - rect.Top;
        if (width <= 0) width = 640;
        if (height <= 0) height = 490;

        using (Bitmap bmp = new Bitmap(width, height)) {
            using (Graphics g = Graphics.FromImage(bmp)) {
                IntPtr hdc = g.GetHdc();
                PrintWindow(hWnd, hdc, 2);
                g.ReleaseHdc(hdc);
            }
            bmp.Save(outputPath, ImageFormat.Png);
        }
    }
}
"@ -ReferencedAssemblies System.Drawing

function Capture-CompactGui {
    $p = Start-Process -FilePath "C:\IPNBS\dist\vendor\MitraAudio-SN-Generator\MitraAudio-SN-Generator.exe" -PassThru
    Start-Sleep -Seconds 2
    $handle = (Get-Process -Id $p.Id).MainWindowHandle

    if ($handle -ne [IntPtr]::Zero) {
        [WindowSnapshot]::CaptureWindow($handle, "C:\IPNBS\docs\poc\license-generator-gui\screenshots\01_vendor_sn_generator_compact_dark.png")
        Write-Output "SUCCESS: Captured compact GUI (Handle: $handle)"
    } else {
        Write-Output "FAIL: MainWindowHandle is Zero"
    }

    Stop-Process -Id $p.Id -Force
}

Capture-CompactGui
