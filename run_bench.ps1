$iterations = 30
$outputFile = "result.txt"


$utf8 = [System.Text.UTF8Encoding]::new($false)
$OutputEncoding = $utf8
[Console]::OutputEncoding = $utf8
[Console]::InputEncoding = $utf8


if (Test-Path $outputFile) {
    Remove-Item $outputFile
}

for ($i = 1; $i -le $iterations; $i++) {
    Write-Host "Running iteration $i of $iterations..."
    
    & .\target\release\runner.exe -k bench 2>&1 | Out-File -Append -FilePath $outputFile -Encoding utf8
    "" | Out-File -Append -FilePath $outputFile -Encoding utf8
}

Write-Host "Done! Results saved to $outputFile"
