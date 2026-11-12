Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public class WinApiTester {
    [DllImport("user32.dll")]
    public static extern IntPtr GetDlgItem(IntPtr h, int id);
    [DllImport("user32.dll")]
    public static extern int SendMessage(IntPtr h, int m, int w, int l);
    [DllImport("user32.dll", EntryPoint="SendMessageA")]
    public static extern int SendMessage(IntPtr h, int m, int w, StringBuilder l);
}
"@

$p = Start-Process "C:\IPNBS\dist\vendor\MitraAudio-SN-Generator\MitraAudio-SN-Generator.exe" -PassThru
Start-Sleep -Seconds 1
$hwnd = $p.MainWindowHandle
$hCombo = [WinApiTester]::GetDlgItem($hwnd, 101)

$count = [WinApiTester]::SendMessage($hCombo, 0x0146, 0, 0)
$cursel = [WinApiTester]::SendMessage($hCombo, 0x0147, 0, 0)

Write-Output "HCOMBO: $hCombo, COUNT: $count, CURSEL: $cursel"

for ($i = 0; $i -lt $count; $i++) {
    $sb = New-Object System.Text.StringBuilder 256
    [WinApiTester]::SendMessage($hCombo, 0x0148, $i, $sb)
    Write-Output "Item $i : $($sb.ToString())"
}

Stop-Process -Id $p.Id -Force
