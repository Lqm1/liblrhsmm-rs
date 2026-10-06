param(
    [Parameter(Mandatory)][string]$SourcePath,
    [Parameter(Mandatory)][string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
$source = Get-Content -LiteralPath $SourcePath -Raw
foreach ($name in @('lse', 'exp')) {
    $match = [regex]::Match($source, '(?s)lrh_' + $name + '_table\[.*?\]\s*=\s*\{(.*?)\};')
    if (-not $match.Success) { throw "Missing upstream $name table" }
    $numbers = $match.Groups[1].Value.Split(',', [System.StringSplitOptions]::RemoveEmptyEntries)
    if ($numbers.Count -ne 7000) { throw "Unexpected upstream $name table length" }
    $path = Join-Path $OutputDirectory ($name + '-table.bin')
    $writer = [System.IO.BinaryWriter]::new([System.IO.File]::Create($path))
    try {
        foreach ($number in $numbers) {
            $writer.Write([float]::Parse($number.Trim(), [System.Globalization.CultureInfo]::InvariantCulture))
        }
    } finally { $writer.Dispose() }
}
