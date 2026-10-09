Write-Host "=== Setting up E2E Testnet Keys ==="

$keys = @("client", "freelancer", "arb1", "arb2", "arb3")
foreach ($k in $keys) {
    Write-Host "Ensuring identity: $k"
    stellar keys generate $k --network testnet --fund 2>$null
    stellar keys fund $k --network testnet 2>$null
}

Write-Host "All testnet identities generated and funded."
Write-Host "Client: $(stellar keys address client)"
Write-Host "Freelancer: $(stellar keys address freelancer)"
Write-Host "Arb1: $(stellar keys address arb1)"
Write-Host "Arb2: $(stellar keys address arb2)"
Write-Host "Arb3: $(stellar keys address arb3)"

Write-Host "E2E key provisioning complete."
