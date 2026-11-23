#Requires -Version 5.1
<#
.SYNOPSIS
    Stop and remove the dok3search Windows Service without deleting DataDir.
#>
[CmdletBinding()]
param(
    [string]$ServiceName = "dok3search",
    [string]$NssmPath = ""
)

$ErrorActionPreference = "Stop"

$svc = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if (-not $svc) {
    Write-Host "Service '$ServiceName' is not installed."
    exit 0
}

if ($svc.Status -ne "Stopped") {
    Stop-Service -Name $ServiceName -Force
    $svc.WaitForStatus("Stopped", [TimeSpan]::FromSeconds(30))
}

if ($NssmPath -and (Test-Path -LiteralPath $NssmPath -PathType Leaf)) {
    & $NssmPath remove $ServiceName confirm | Out-Null
} else {
    sc.exe delete $ServiceName | Out-Null
}

Write-Host "Service '$ServiceName' removed. Operational data directory was preserved."
