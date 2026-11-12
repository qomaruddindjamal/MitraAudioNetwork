Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

function Capture-WindowScreenshot {
    param (
        [string]$ProcessPath,
        [string]$OutputPath,
        [int]$WaitSec = 2
    )

    $proc = Start-Process -FilePath $ProcessPath -PassThru
    Start-Sleep -Seconds $WaitSec

    $bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
    $bitmap = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
    $bitmap.Save($OutputPath, [System.Drawing.Imaging.ImageFormat]::Png)
    $graphics.Dispose()
    $bitmap.Dispose()

    Stop-Process -Id $proc.Id -Force
    Write-Output "Captured: $OutputPath"
}

Capture-WindowScreenshot -ProcessPath "C:\IPNBS\target\debug\mitra-sn-generator.exe" -OutputPath "C:\IPNBS\docs\poc\license-generator-gui\screenshots\01_vendor_sn_generator_dashboard.png"
Capture-WindowScreenshot -ProcessPath "C:\IPNBS\target\debug\mitra-customer-app.exe" -OutputPath "C:\IPNBS\docs\poc\license-generator-gui\screenshots\02_customer_app_initial.png"
