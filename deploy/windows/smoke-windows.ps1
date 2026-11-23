#Requires -Version 5.1
<#
.SYNOPSIS
    Verify dok3search Windows Service health, search API, and restart recovery.
#>
[CmdletBinding()]
param(
    [string]$ServiceName = "dok3search",
    [string]$BaseUrl = "http://127.0.0.1:8080",
    [string]$Query = "rust"
)

$ErrorActionPreference = "Stop"

function Test-Dok3Endpoints {
    param([string]$Url, [string]$SearchQuery)

    $ready = Invoke-WebRequest -Uri "$Url/health/ready" -UseBasicParsing -TimeoutSec 10
    if ($ready.StatusCode -ne 200) {
        throw "/health/ready returned $($ready.StatusCode)"
    }

    $body = @{ query = $SearchQuery; limit = 3 } | ConvertTo-Json -Compress
    $search = Invoke-WebRequest -Uri "$Url/api/v1/search" -Method Post -ContentType "application/json" -Body $body -UseBasicParsing -TimeoutSec 10
    if ($search.StatusCode -ne 200) {
        throw "POST /api/v1/search returned $($search.StatusCode)"
    }
    Write-Host "OK: $Url (/health/ready=200, POST /api/v1/search=200)"
}

Test-Dok3Endpoints -Url $BaseUrl -SearchQuery $Query

Write-Host "Restarting Windows Service '$ServiceName'..."
Restart-Service -Name $ServiceName -Force
$svc = Get-Service -Name $ServiceName
$svc.WaitForStatus("Running", [TimeSpan]::FromSeconds(30))
Start-Sleep -Seconds 2

Test-Dok3Endpoints -Url $BaseUrl -SearchQuery $Query
Write-Host "Windows Service smoke + restart recovery PASSED"
