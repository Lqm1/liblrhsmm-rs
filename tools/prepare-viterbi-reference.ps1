param(
    [Parameter(Mandatory)][string]$SourceDirectory,
    [Parameter(Mandatory)][string]$OutputDirectory
)

$ErrorActionPreference = 'Stop'
$sourceRoot = (Resolve-Path -LiteralPath $SourceDirectory).Path
$outputRoot = [System.IO.Path]::GetFullPath($OutputDirectory)
if ($sourceRoot.TrimEnd('\', '/') -eq $outputRoot.TrimEnd('\', '/')) {
    throw 'Reference output must differ from the upstream source directory.'
}
$null = New-Item -ItemType Directory -Path $outputRoot -Force
Copy-Item -LiteralPath (Join-Path $sourceRoot 'inference.c') -Destination (Join-Path $outputRoot 'inference.c')
$header = [System.IO.File]::ReadAllText((Join-Path $sourceRoot 'inference-forward.h')).Replace("`r`n", "`n")
$original = @'
  if(forward != NULL)
    memcpy(forward, a_begin, nseg * nt * sizeof(FP_TYPE));
'@
$replacement = @'
  extern void lrh_reference_viterbi_scores(const FP_TYPE*, const FP_TYPE*, int, int);
  lrh_reference_viterbi_scores(a_begin, a_end, nt, nseg);
  if(forward != NULL) {
    *forward = malloc(nseg * nt * sizeof(FP_TYPE));
    memcpy(*forward, a_begin, nseg * nt * sizeof(FP_TYPE));
  }
'@
if ([regex]::Matches($header, [regex]::Escape($original)).Count -ne 1) {
    throw 'Expected exactly one original Viterbi output branch.'
}
$header = $header.Replace($original, $replacement)
[System.IO.File]::WriteAllText((Join-Path $outputRoot 'inference-forward.h'), $header)
