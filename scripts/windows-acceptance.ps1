# Windows synthetic acceptance harness for MLAD OFFLINE/BENCH.
# NOT an automated WebView2 UI test. Never accesses robot hardware.
[CmdletBinding()]
param(
    [ValidateSet("Prepare", "Verify", "ValidateBinary")]
    [string]$Mode = "Prepare",
    [string]$FixtureRoot = "",
    [string]$Executable = "",
    [string]$EvidenceDir = ""
)
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Require-Value([string]$Value, [string]$Name) {
    if ([string]::IsNullOrWhiteSpace($Value)) { throw "Specify -$Name for mode $Mode" }
}
function Write-Json([string]$Path, [object]$Value) {
    $json = $Value | ConvertTo-Json -Depth 8
    [System.IO.File]::WriteAllText($Path, $json + [Environment]::NewLine,
        [System.Text.UTF8Encoding]::new($false))
}
function Assert-SyntheticPath([string]$Relative) {
    if ($Relative -cnotmatch '^native/(CIOPRG\.LST|IONAME\.DAT)$') {
        throw "Unsupported fixture path in manifest: $Relative"
    }
}

if ($Mode -eq "Prepare") {
    Require-Value $FixtureRoot "FixtureRoot"
    if (Test-Path -LiteralPath $FixtureRoot) {
        throw "Fixture directory already exists; refuse to overwrite: $FixtureRoot"
    }
    $nativeDir = Join-Path $FixtureRoot "native"
    $draftDir = Join-Path $FixtureRoot "drafts"
    [void][System.IO.Directory]::CreateDirectory($nativeDir)
    [void][System.IO.Directory]::CreateDirectory($draftDir)

    # Deliberately synthetic demonstration text; NOT an OEM-valid program.
    $crlf = [string][char]13 + [string][char]10
    $cioprgText = @(
        "; MLAD PR28 SYNTHETIC ONLY - NOT AN OEM PROGRAM",
        "PART 1",
        "STR #70010",
        "OUT #70011",
        "END",
        "ALARM SYNTHETIC_TAIL_NOT_LADDER",
        ""
    ) -join $crlf
    $ionameText = @(
        "; MLAD PR28 SYNTHETIC IONAME - NOT AN OEM ADDRESS MAP",
        "#70010,TEST_INPUT",
        "#70011,TEST_OUTPUT",
        ""
    ) -join $crlf
    [System.IO.File]::WriteAllBytes((Join-Path $nativeDir "CIOPRG.LST"),
        [System.Text.Encoding]::ASCII.GetBytes($cioprgText))
    [System.IO.File]::WriteAllBytes((Join-Path $nativeDir "IONAME.DAT"),
        [System.Text.Encoding]::ASCII.GetBytes($ionameText))

    $files = @()
    foreach ($name in @("CIOPRG.LST", "IONAME.DAT")) {
        $file = Join-Path $nativeDir $name
        $files += [pscustomobject]@{
            relative = "native/$name"
            bytes = (Get-Item -LiteralPath $file).Length
            sha256 = (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash
        }
    }
    Write-Json (Join-Path $FixtureRoot "manifest.json") ([pscustomobject]@{
        schema = "mlad-pr28-synthetic-v1"
        warning = "Synthetic only. Not a real Yaskawa program, not a controller verification."
        files = $files
    })
    $readme = @(
        "MLAD PR28 synthetic fixtures (NOT OEM-valid).",
        "Open only as OFFLINE read-only demonstration; parse errors are expected.",
        "Do not connect MLAD to a production robot or robot network.",
        "After review: pwsh -File scripts/windows-acceptance.ps1 -Mode Verify -FixtureRoot $FixtureRoot",
        ""
    ) -join $crlf
    [System.IO.File]::WriteAllText((Join-Path $FixtureRoot "README.txt"), $readme,
        [System.Text.UTF8Encoding]::new($false))
    Write-Output "PREPARED synthetic acceptance fixture: $FixtureRoot"
    exit 0
}

if ($Mode -eq "Verify") {
    Require-Value $FixtureRoot "FixtureRoot"
    $manifestPath = Join-Path $FixtureRoot "manifest.json"
    if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
        throw "Synthetic fixture manifest not found: $manifestPath"
    }
    $manifest = Get-Content -Raw -LiteralPath $manifestPath | ConvertFrom-Json
    if ($manifest.schema -ne "mlad-pr28-synthetic-v1" -or @($manifest.files).Count -ne 2) {
        throw "Invalid synthetic fixture manifest"
    }
    foreach ($entry in $manifest.files) {
        Assert-SyntheticPath ([string]$entry.relative)
        $relative = ([string]$entry.relative).Replace("/", [IO.Path]::DirectorySeparatorChar)
        $path = Join-Path $FixtureRoot $relative
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Missing original fixture: $path"
        }
        $actualHash = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash
        $actualLength = (Get-Item -LiteralPath $path).Length
        if ($actualHash -cne [string]$entry.sha256 -or $actualLength -ne [long]$entry.bytes) {
            throw "FAIL: original synthetic file changed: $($entry.relative)"
        }
        Write-Output "PASS immutable original: $($entry.relative) [$actualLength bytes]"
    }
    # Checks original bytes only. UI behavior remains MANUAL/NOT RUN.
    Write-Output "PASS synthetic source integrity. UserSmoke=NOT RUN by this harness; OEMBench=NOT RUN."
    exit 0
}

if ($Mode -eq "ValidateBinary") {
    Require-Value $Executable "Executable"
    Require-Value $EvidenceDir "EvidenceDir"
    if (-not (Test-Path -LiteralPath $Executable -PathType Leaf)) {
        throw "Compiled executable not found: $Executable"
    }
    $file = Get-Item -LiteralPath $Executable
    if ($file.Length -lt 100000) { throw "Binary too small for a Tauri executable" }
    $stream = [System.IO.File]::OpenRead($file.FullName)
    try {
        if ($stream.ReadByte() -ne 77 -or $stream.ReadByte() -ne 90) {
            throw "Missing Windows PE MZ signature"
        }
    } finally { $stream.Dispose() }
    [void][System.IO.Directory]::CreateDirectory($EvidenceDir)
    $signature = Get-AuthenticodeSignature -LiteralPath $file.FullName
    $hash = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash
    $report = [pscustomobject]@{
        schema = "mlad-pr28-binary-evidence-v1"
        generated_utc = [DateTime]::UtcNow.ToString("o")
        git_sha = $env:GITHUB_SHA
        binary_name = $file.Name
        binary_size_bytes = $file.Length
        sha256 = $hash
        authenticode = [string]$signature.Status
        binary_pe_header = "PASS"
        windows_ci_tests = "SEE_GITHUB_ACTIONS_RUN"
        user_webview2_smoke = "NOT RUN"
        oem_bench = "NOT RUN"
        robot_network_or_writer = "BLOCKED"
        release = "NOT PUBLISHED"
    }
    Write-Json (Join-Path $EvidenceDir "binary-evidence.json") $report
    [System.IO.File]::WriteAllText((Join-Path $EvidenceDir "SHA256SUMS.txt"),
        "$hash  $($file.Name)" + [Environment]::NewLine,
        [System.Text.UTF8Encoding]::new($false))
    Write-Output "PASS Windows PE binary existence, size and MZ signature. WebView2 UserSmoke=NOT RUN."
    exit 0
}
throw "Unknown mode"
