Add-Type @"
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;

public class WindowTester {
    [DllImport("user32.dll")]
    public static extern bool PrintWindow(IntPtr hWnd, IntPtr hdcBkgnd, uint nFlags);

    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);

    [DllImport("user32.dll")]
    public static extern IntPtr GetDlgItem(IntPtr hDlg, int nIDDlgItem);

    [DllImport("user32.dll")]
    public static extern IntPtr SendMessage(IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);

    [DllImport("user32.dll", EntryPoint="SetWindowTextA")]
    public static extern bool SetWindowText(IntPtr hWnd, string lpString);

    [StructLayout(LayoutKind.Sequential)]
    public struct RECT {
        public int Left;
        public int Top;
        public int Right;
        public int Bottom;
    }

    public static void SetText(IntPtr hWnd, string text) {
        SetWindowText(hWnd, text);
    }

    public static void ClickButton(IntPtr hBtn) {
        SendMessage(hBtn, 0x00F5 /* BM_CLICK */, IntPtr.Zero, IntPtr.Zero);
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

function Test-CompactWorkflow {
    $p = Start-Process -FilePath "C:\IPNBS\dist\vendor\MitraAudio-SN-Generator\MitraAudio-SN-Generator.exe" -PassThru
    Start-Sleep -Seconds 2
    $handle = (Get-Process -Id $p.Id).MainWindowHandle

    $hEditNative = [WindowTester]::GetDlgItem($handle, 102)
    $hBtnGen = [WindowTester]::GetDlgItem($handle, 105)

    # Set sample Native ID
    [WindowTester]::SetText($hEditNative, "WIN-HWID-V1:MB-TEST-DELL-OPTIPLEX:CPU-I7-12700:DISK-NVME-SAMSUNG")
    Start-Sleep -Milliseconds 500

    # Click GENERATE SN
    [WindowTester]::ClickButton($hBtnGen)
    Start-Sleep -Milliseconds 500

    # Capture state with SN generated and Export License button visible
    [WindowTester]::CaptureWindow($handle, "C:\IPNBS\docs\poc\license-generator-gui\screenshots\02_vendor_sn_generator_generated_state.png")
    Write-Output "SUCCESS: Generated SN workflow captured"

    Stop-Process -Id $p.Id -Force
}

Test-CompactWorkflow
