Add-Type @"
using System;
using System.Runtime.InteropServices;

public class NativeTester {
    [DllImport("user32.dll")]
    public static extern IntPtr SendMessage(IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern IntPtr GetDlgItem(IntPtr hDlg, int nIDDlgItem);
}
"@

$p = Start-Process -FilePath "C:\MitraAudio\SNGenerator\Windows\MitraAudio-SN-Generator.exe" -PassThru
Start-Sleep -Seconds 1
$handle = (Get-Process -Id $p.Id).MainWindowHandle

# Switch to SPON mode
$hCombo = [NativeTester]::GetDlgItem($handle, 101)
[NativeTester]::SendMessage($hCombo, 0x014E, [IntPtr]1, [IntPtr]0)
[NativeTester]::SendMessage($handle, 0x0111, [IntPtr]((1 -shl 16) -bor 101), $hCombo)
Start-Sleep -Milliseconds 300

# Click Run Compatibility Checks (ID 202)
[NativeTester]::SendMessage($handle, 0x0111, [IntPtr]202, [IntPtr]0)
Start-Sleep -Milliseconds 300

# Click Export Test Report (ID 203)
# To avoid blocking on MessageBox, post message or execute
[NativeTester]::SendMessage($handle, 0x0111, [IntPtr]203, [IntPtr]0)
Start-Sleep -Milliseconds 500

Stop-Process -Id $p.Id -Force
