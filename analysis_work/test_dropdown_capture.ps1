Add-Type @"
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;

public class DropdownTestHelper {
    [DllImport("user32.dll")]
    public static extern IntPtr GetDlgItem(IntPtr h, int id);
    [DllImport("user32.dll")]
    public static extern int SendMessage(IntPtr h, int m, int w, int l);
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

    public static void CaptureWithDroppedCombo(IntPtr hMain, IntPtr hCombo, string outPath) {
        // Send CB_SHOWDROPDOWN = 0x014F with TRUE
        SendMessage(hCombo, 0x014F, 1, 0);
        System.Threading.Thread.Sleep(300);

        using (Bitmap bmp = new Bitmap(640, 490)) {
            using (Graphics g = Graphics.FromImage(bmp)) {
                IntPtr hdc = g.GetHdc();
                PrintWindow(hMain, hdc, 2);
                g.ReleaseHdc(hdc);
            }
            bmp.Save(outPath, ImageFormat.Png);
        }
    }
}
"@ -ReferencedAssemblies System.Drawing

$p = Start-Process "C:\IPNBS\dist\vendor\MitraAudio-SN-Generator\MitraAudio-SN-Generator.exe" -PassThru
Start-Sleep -Seconds 1
$hwnd = $p.MainWindowHandle
$hCombo = [DropdownTestHelper]::GetDlgItem($hwnd, 101)

[DropdownTestHelper]::CaptureWithDroppedCombo($hwnd, $hCombo, "C:\IPNBS\docs\poc\license-generator-gui\screenshots\test_dropdown_open.png")
Stop-Process -Id $p.Id -Force
Write-Output "Captured test_dropdown_open.png"
