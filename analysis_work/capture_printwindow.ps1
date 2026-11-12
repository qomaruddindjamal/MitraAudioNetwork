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
        if (width <= 0) width = 960;
        if (height <= 0) height = 700;

        using (Bitmap bmp = new Bitmap(width, height)) {
            using (Graphics g = Graphics.FromImage(bmp)) {
                IntPtr hdc = g.GetHdc();
                PrintWindow(hWnd, hdc, 2); // PW_RENDERFULLCONTENT
                g.ReleaseHdc(hdc);
            }
            bmp.Save(outputPath, ImageFormat.Png);
        }
    }
}
"@ -ReferencedAssemblies System.Drawing

function Capture-App {
    param([string]$ExePath, [string]$OutPath)
    $p = Start-Process -FilePath $ExePath -PassThru
    Start-Sleep -Seconds 2
    $handle = (Get-Process -Id $p.Id).MainWindowHandle
    if ($handle -ne [IntPtr]::Zero) {
        [WindowSnapshot]::CaptureWindow($handle, $OutPath)
        Write-Output "SUCCESS: $OutPath (Handle: $handle)"
    } else {
        Write-Output "FAIL: MainWindowHandle is Zero"
    }
    Stop-Process -Id $p.Id -Force
}

Capture-App "C:\IPNBS\target\debug\mitra-sn-generator.exe" "C:\IPNBS\docs\poc\license-generator-gui\screenshots\01_vendor_sn_generator_dashboard.png"
Capture-App "C:\IPNBS\target\debug\mitra-customer-app.exe" "C:\IPNBS\docs\poc\license-generator-gui\screenshots\02_customer_app_initial.png"
