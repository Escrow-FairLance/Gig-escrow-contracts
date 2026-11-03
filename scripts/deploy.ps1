Write-Host "=== 1. Running cargo test ==="
cargo test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "=== 2. Ensuring deployer key on testnet ==="
stellar keys generate deployer --network testnet --fund 2>$null
stellar keys fund deployer --network testnet 2>$null

Write-Host "=== 3. Building contract wasm ==="
stellar contract build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$wasmPath = "target/wasm32-unknown-unknown/release/gig_escrow_contracts.wasm"
if (-not (Test-Path $wasmPath)) {
    $wasmPath = "target/wasm32v1-none/release/gig_escrow_contracts.wasm"
}

Write-Host "=== 4. Deploying contract to testnet ==="
$contractId = stellar contract deploy --wasm $wasmPath --source deployer --network testnet --alias fairlance 2>$null

$tokenId = stellar contract id asset --asset native --network testnet
$deployerAddr = stellar keys address deployer

$config = @{
    network = "testnet"
    contractId = if ($contractId) { $contractId.Trim() } else { "CCFJLJRJWMA6EG3N3X3EK7UN3OPKN3YKOAR4T53KSXOSRDJWT473HMAY" }
    contractAlias = "fairlance"
    nativeTokenContractId = $tokenId.Trim()
    deployerAddress = $deployerAddr.Trim()
    rpcUrl = "https://soroban-testnet.stellar.org"
    networkPassphrase = "Test SDF Network ; September 2015"
}

$config | ConvertTo-Json -Depth 5 | Out-File -FilePath "contract-ids.json" -Encoding utf8
Write-Host "Deployment metadata updated in contract-ids.json"
