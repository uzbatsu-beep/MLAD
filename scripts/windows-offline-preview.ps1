# PR48 deterministic Windows Offline Preview packaging + integrity verifier.
# Never tests WebView2 GUI/OEM execution, never accesses the robot.
[CmdletBinding()]
param(
    [ValidateSet("Package","Verify")][string]$Mode="Verify",
    [Parameter(Mandatory=$true)][string]$StageRoot,
    [string]$Executable,
    [string]$FixtureRoot,
    [string]$RepositoryRoot=".",
    [string]$ExpectedGitSha=""
)
Set-StrictMode -Version Latest
$ErrorActionPreference="Stop"

function Require([bool]$ok,[string]$message){
    if(-not $ok){throw $message}
}
function SHA([string]$path){
    return (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
}
function Rel([string]$root,[string]$path){
    return [IO.Path]::GetRelativePath($root,$path).Replace("\","/")
}
function WriteJSON([string]$path,[object]$value){
    [IO.File]::WriteAllText($path,(ConvertTo-Json -InputObject $value -Depth 12)+[Environment]::NewLine,
        [Text.UTF8Encoding]::new($false))
}
function Verify-Preview([string]$stage){
    Require (Test-Path -LiteralPath $stage -PathType Container) "Preview staging directory missing"
    $root=(Resolve-Path -LiteralPath $stage).Path
    $sumFile=Join-Path $root "SHA256SUMS.txt"
    Require (Test-Path -LiteralPath $sumFile -PathType Leaf) "Missing SHA256SUMS.txt"
    $items=@{}
    foreach($line in [IO.File]::ReadAllLines($sumFile)){
        if($line -cnotmatch '^([a-f0-9]{64})  ([A-Za-z0-9._/-]+)$'){
            throw "Malformed package SHA entry"
        }
        $hash=$Matches[1];$relative=$Matches[2]
        if($relative.StartsWith("/") -or $relative.Split("/") -contains ".." -or
           $relative.Split("/") -contains "." -or $items.ContainsKey($relative)){
            throw "Invalid/duplicate relative package filename"
        }
        $target=Join-Path $root $relative.Replace("/",[IO.Path]::DirectorySeparatorChar)
        Require (Test-Path -LiteralPath $target -PathType Leaf) "Missing package member: $relative"
        Require ((SHA $target) -ceq $hash) "Package SHA mismatch: $relative"
        $items[$relative]=$hash
    }
    $actual=@(Get-ChildItem -LiteralPath $root -Recurse -File | Where-Object {
        $_.FullName -ne $sumFile
    } | ForEach-Object{Rel $root $_.FullName})
    Require ($actual.Count -eq $items.Count) "Unexpected unlisted package files"
    foreach($f in $actual){Require ($items.ContainsKey($f)) "Unlisted package member: $f"}
    foreach($f in @(
        "app/yaskawa-ladder-ide.exe","sample/CIOPRG.LST","sample/IONAME.DAT",
        "sample/cell.mlad.json","sample/cell.mladproj.json",
        "sample/SYNTHETIC-SHA256.json","tools/windows-offline-preview.ps1",
        "tools/offline-preview-fixture.py","docs/WINDOWS-OFFLINE-USER-SMOKE.md",
        "LICENSE","corresponding-source.zip","build-evidence.json"
    )){
        Require ($items.ContainsKey($f)) "Required asset missing: $f"
    }
    $exe=Join-Path $root "app/yaskawa-ladder-ide.exe"
    $stream=[IO.File]::OpenRead($exe)
    try{
        Require ($stream.Length -gt 100000) "Unexpectedly short EXE"
        Require ($stream.ReadByte() -eq 77 -and $stream.ReadByte() -eq 90) "EXE missing PE MZ"
    } finally {$stream.Dispose()}
    $guard=Get-Content -Raw -LiteralPath (Join-Path $root "sample/SYNTHETIC-SHA256.json") | ConvertFrom-Json
    Require ($guard.schema -eq "MLAD_PR48_SYNTHETIC_FIXTURE_V1") "Wrong fixture version"
    Require ($guard.source_kind -eq "SYNTHETIC_NOT_OEM") "Fixture provenance failure"
    Require ($guard.user_smoke -eq "NOT RUN" -and $guard.oem_bench -eq "NOT RUN") "Automated smoke claims forbidden"
    Require (@($guard.files).Count -eq 4) "Incomplete fixture inventory"
    foreach($f in @($guard.files)){
        $name=[string]$f.relative_path
        Require ($name -in @("CIOPRG.LST","IONAME.DAT","cell.mlad.json","cell.mladproj.json")) "Wrong fixture name"
        $target=Join-Path $root ("sample/"+$name)
        Require ((SHA $target) -ceq [string]$f.sha256) "Synthetic member changed: $name"
        Require ((Get-Item -LiteralPath $target).Length -eq [long]$f.bytes) "Synthetic byte count differs"
    }
    $project=Get-Content -Raw -LiteralPath (Join-Path $root "sample/cell.mladproj.json") | ConvertFrom-Json
    $notes=Get-Content -Raw -LiteralPath (Join-Path $root "sample/cell.mlad.json") | ConvertFrom-Json
    Require ($project.format -eq "MLAD_OFFLINE_PROJECT" -and $project.schema_version -eq 1) "Project schema failure"
    Require ($notes.format -eq "MLAD_OFFLINE_SIDECAR" -and $notes.schema_version -eq 1) "Sidecar schema failure"
    Require ($project.source.sha256 -ceq (SHA (Join-Path $root "sample/CIOPRG.LST"))) "Native source unpinned"
    Require ($project.ioname.sha256 -ceq (SHA (Join-Path $root "sample/IONAME.DAT"))) "IONAME unpinned"
    Require ($project.sidecar.sha256 -ceq (SHA (Join-Path $root "sample/cell.mlad.json"))) "Sidecar unpinned"
    Require ($notes.source.sha256 -ceq $project.source.sha256) "Sidecar source mismatch"
    Require ($project.analysis.execution_verified -eq $false) "Unverified execution may not be asserted"
    $evidence=Get-Content -Raw -LiteralPath (Join-Path $root "build-evidence.json") | ConvertFrom-Json
    Require ($evidence.schema -eq "MLAD_PR48_WINDOWS_OFFLINE_PREVIEW_V1") "Wrong build evidence schema"
    Require ($evidence.executable_sha256 -ceq (SHA $exe)) "Evidence not tied to executable"
    Require ($evidence.git_sha -cmatch '^[0-9a-f]{40}$') "Build source commit absent"
    Require ($evidence.user_smoke -eq "NOT RUN" -and $evidence.oem_bench -eq "NOT RUN") "Manual tests not performed"
    Require ($evidence.release -eq "NOT PUBLISHED" -and $evidence.robot_access -eq "BLOCKED") "Unsafe status"
    Write-Output "PASS PR48 preview package: SHA, PE, synthetic bindings, source revision"
    Write-Output "GUI UserSmoke=NOT RUN; OEMBench=NOT RUN"
}

if($Mode -eq "Verify"){
    Verify-Preview $StageRoot
    exit 0
}
Require ($Executable -and (Test-Path -LiteralPath $Executable -PathType Leaf)) "Compiled EXE required"
Require ($FixtureRoot -and (Test-Path -LiteralPath $FixtureRoot -PathType Container)) "Fixture directory required"
Require (-not (Test-Path -LiteralPath $StageRoot)) "Refusing to overwrite a staged preview"
$repo=(Resolve-Path -LiteralPath $RepositoryRoot).Path
$sha=(& git -C $repo rev-parse HEAD).Trim()
Require ($LASTEXITCODE -eq 0 -and $sha -cmatch '^[0-9a-f]{40}$') "Git HEAD unavailable"
if($ExpectedGitSha){Require ($sha -ceq $ExpectedGitSha) "Checkout differs from expected PR HEAD"}
$exeBytes=[IO.File]::ReadAllBytes($Executable)
Require ($exeBytes.Length -gt 100000 -and $exeBytes[0] -eq 77 -and $exeBytes[1] -eq 90) "Invalid PE executable"
$stage=[IO.Path]::GetFullPath($StageRoot)
foreach($f in @("app","sample","tools","docs")){
    New-Item -ItemType Directory -Path (Join-Path $stage $f) -Force | Out-Null
}
Copy-Item -LiteralPath $Executable -Destination (Join-Path $stage "app/yaskawa-ladder-ide.exe")
foreach($f in @("CIOPRG.LST","IONAME.DAT","cell.mlad.json","cell.mladproj.json","SYNTHETIC-SHA256.json")){
    Copy-Item -LiteralPath (Join-Path $FixtureRoot $f) -Destination (Join-Path $stage ("sample/"+$f))
}
Copy-Item -LiteralPath (Join-Path $repo "scripts/offline-preview-fixture.py") -Destination (Join-Path $stage "tools/offline-preview-fixture.py")
Copy-Item -LiteralPath (Join-Path $repo "scripts/windows-offline-preview.ps1") -Destination (Join-Path $stage "tools/windows-offline-preview.ps1")
Copy-Item -LiteralPath (Join-Path $repo "docs/windows_offline_preview_pr48.md") -Destination (Join-Path $stage "docs/WINDOWS-OFFLINE-USER-SMOKE.md")
Copy-Item -LiteralPath (Join-Path $repo "LICENSE") -Destination (Join-Path $stage "LICENSE")
$sourceZip=Join-Path $stage "corresponding-source.zip"
& git -C $repo archive --format=zip -o $sourceZip HEAD
Require ($LASTEXITCODE -eq 0 -and (Test-Path -LiteralPath $sourceZip)) "GPL corresponding-source zip missing"
$signature=Get-AuthenticodeSignature -LiteralPath $Executable
WriteJSON (Join-Path $stage "build-evidence.json") ([ordered]@{
    schema="MLAD_PR48_WINDOWS_OFFLINE_PREVIEW_V1"
    generated_utc=[DateTime]::UtcNow.ToString("o")
    git_sha=$sha
    github_run_id=[string]$env:GITHUB_RUN_ID
    executable_sha256=(SHA (Join-Path $stage "app/yaskawa-ladder-ide.exe"))
    executable_authenticode=[string]$signature.Status
    windows_ci="SEE_GITHUB_ACTIONS_RUN"
    user_smoke="NOT RUN"
    oem_bench="NOT RUN"
    release="NOT PUBLISHED"
    robot_access="BLOCKED"
})
$lines=@(Get-ChildItem -LiteralPath $stage -Recurse -File | Sort-Object FullName |
    ForEach-Object{(SHA $_.FullName)+"  "+(Rel $stage $_.FullName)})
[IO.File]::WriteAllText((Join-Path $stage "SHA256SUMS.txt"),
    ($lines -join [Environment]::NewLine)+[Environment]::NewLine,[Text.UTF8Encoding]::new($false))
Verify-Preview $stage
Write-Output "PACKAGED unsigned Windows preview at $stage, exact source HEAD $sha"
