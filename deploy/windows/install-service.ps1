#Requires -Version 5.1
<#
.SYNOPSIS
    Install and start dok3search as a Windows Service with automatic restart recovery.
#>
[CmdletBinding()]
param(
    [string]$ServiceName = "dok3search",
    [Parameter(Mandatory = $true)]
    [string]$BinaryPath,
    [string]$Bind = "127.0.0.1",
    [int]$Port = 8080,
    [Parameter(Mandatory = $true)]
    [string]$SourceDir,
    [Parameter(Mandatory = $true)]
    [string]$DataDir,
    [string]$StaticDir = "",
    [string]$AnalyzerProfile = "builtin",
    [string]$DictPath = "",
    [string]$NssmPath = ""
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw "BinaryPath not found: $BinaryPath"
}
if (-not (Test-Path -LiteralPath $SourceDir -PathType Container)) {
    throw "SourceDir not found: $SourceDir"
}

New-Item -ItemType Directory -Force -Path $DataDir | Out-Null
$logDir = Join-Path $DataDir "logs"
New-Item -ItemType Directory -Force -Path $logDir | Out-Null

$envLines = @(
    "DOK3_BIND=$Bind"
    "DOK3_PORT=$Port"
    "DOK3_SOURCE_DIR=$SourceDir"
    "DOK3_DATA_DIR=$DataDir"
    "DOK3_ANALYZER_PROFILE=$AnalyzerProfile"
)
if ($StaticDir) { $envLines += "DOK3_STATIC_DIR=$StaticDir" }
if ($DictPath)  { $envLines += "DOK3_DICT_PATH=$DictPath" }

# Persist machine-level environment variables for sc.exe / native service runs.
foreach ($line in $envLines) {
    $parts = $line -split "=", 2
    [Environment]::SetEnvironmentVariable($parts[0], $parts[1], "Machine")
}

if ($NssmPath -and (Test-Path -LiteralPath $NssmPath -PathType Leaf)) {
    & $NssmPath install $ServiceName $BinaryPath | Out-Null
    & $NssmPath set $ServiceName AppDirectory (Split-Path -Parent $BinaryPath) | Out-Null
    & $NssmPath set $ServiceName AppStdout (Join-Path $logDir "dok3search-stdout.log") | Out-Null
    & $NssmPath set $ServiceName AppStderr (Join-Path $logDir "dok3search-stderr.log") | Out-Null
    & $NssmPath set $ServiceName AppEnvironmentExtra ($envLines -join "`n") | Out-Null
    & $NssmPath set $ServiceName Start SERVICE_AUTO_START | Out-Null
} else {
    $existing = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    if (-not $existing) {
        sc.exe create $ServiceName binPath= "`"$BinaryPath`"" start= auto DisplayName= "dok3search Knowledge Server" | Out-Null
    }
    sc.exe description $ServiceName "Air-gapped document and code search platform (dok3search)" | Out-Null
    # Restart after 5s / 15s / 30s on failure; reset counter after 1 day (86400s).
    sc.exe failure $ServiceName reset= 86400 actions= restart/5000/restart/15000/restart/30000 | Out-Null
}

Start-Service -Name $ServiceName
$svc = Get-Service -Name $ServiceName
Write-Host "Service '$ServiceName' status: $($svc.Status)"
