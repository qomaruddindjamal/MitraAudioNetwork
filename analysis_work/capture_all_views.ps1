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

    [DllImport("user32.dll")]
    public static extern IntPtr SendMessage(IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);

    [StructLayout(LayoutKind.Sequential)]
    public struct RECT {
        public int Left;
        public int Top;
        public int Right;
        public int Bottom;
    }

    public static void SendCommand(IntPtr hWnd, int cmdId) {
        SendMessage(hWnd, 0x0111 /* WM_COMMAND */, (IntPtr)cmdId, IntPtr.Zero);
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
                PrintWindow(hWnd, hdc, 2);
                g.ReleaseHdc(hdc);
            }
            bmp.Save(outputPath, ImageFormat.Png);
        }
    }
}
"@ -ReferencedAssemblies System.Drawing

function Capture-Flow {
    $p = Start-Process -FilePath "C:\IPNBS\target\debug\mitra-sn-generator.exe" -PassThru
    Start-Sleep -Seconds 2
    $handle = (Get-Process -Id $p.Id).MainWindowHandle

    # 1. Dashboard (Already captured)
    
    # 2. Switch to SN Generator (ID_NAV_GENERATOR = 102)
    [WindowSnapshot]::SendCommand($handle, 102)
    Start-Sleep -Milliseconds 500
    [WindowSnapshot]::CaptureWindow($handle, "C:\IPNBS\docs\poc\license-generator-gui\screenshots\03_vendor_sn_generator_view.png")

    # 3. Switch to License Issuer (ID_NAV_ISSUER = 103)
    [WindowSnapshot]::SendCommand($handle, 103)
    Start-Sleep -Milliseconds 500
    [WindowSnapshot]::CaptureWindow($handle, "C:\IPNBS\docs\poc\license-generator-gui\screenshots\04_vendor_license_issuer_view.png")

    # 4. Switch to Verification (ID_NAV_VERIFY = 104)
    [WindowSnapshot]::SendCommand($handle, 104)
    Start-Sleep -Milliseconds 500
    [WindowSnapshot]::CaptureWindow($handle, "C:\IPNBS\docs\poc\license-generator-gui\screenshots\05_vendor_license_verification_view.png")

    # 5. Switch to Support (ID_NAV_SUPPORT = 107)
    [WindowSnapshot]::SendCommand($handle, 107)
    Start-Sleep -Milliseconds 500
    [WindowSnapshot]::CaptureWindow($handle, "C:\IPNBS\docs\poc\license-generator-gui\screenshots\06_vendor_support_replacement_view.png")

    Stop-Process -Id $p.Id -Force
    Write-Output "All Views Captured Successfully"
}

Capture-Flow
