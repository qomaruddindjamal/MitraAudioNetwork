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

    [DllImport("user32.dll")]
    public static extern IntPtr GetDlgItem(IntPtr hDlg, int nIDDlgItem);

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
        if (width <= 0) width = 660;
        if (height <= 0) height = 590;

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

$outDir = "C:\MitraAudio\docs\poc\license-generator-gui\screenshots"
if (!(Test-Path $outDir)) { New-Item -ItemType Directory -Force $outDir | Out-Null }

$p = Start-Process -FilePath "C:\MitraAudio\SNGenerator\Windows\MitraAudio-SN-Generator.exe" -PassThru
Start-Sleep -Seconds 1
$handle = (Get-Process -Id $p.Id).MainWindowHandle

if ($handle -ne [IntPtr]::Zero) {
    # 1. Capture Default Mitra Audio View with Logo
    [WindowSnapshot]::CaptureWindow($handle, "$outDir\01_mitra_audio_unified_generator.png")
    Write-Output "SUCCESS: 01_mitra_audio_unified_generator.png"

    # 2. Switch to SPON mode
    $hCombo = [WindowSnapshot]::GetDlgItem($handle, 101)
    [WindowSnapshot]::SendMessage($hCombo, 0x014E, [IntPtr]1, [IntPtr]0) # CB_SETCURSEL = 1
    [WindowSnapshot]::SendMessage($handle, 0x0111, [IntPtr]((1 -shl 16) -bor 101), $hCombo) # WM_COMMAND CBN_SELCHANGE
    Start-Sleep -Milliseconds 500

    [WindowSnapshot]::CaptureWindow($handle, "$outDir\02_spon_native_compatibility_panel.png")
    Write-Output "SUCCESS: 02_spon_native_compatibility_panel.png"
} else {
    Write-Output "FAIL: Process MainWindowHandle is zero"
}

Stop-Process -Id $p.Id -Force
