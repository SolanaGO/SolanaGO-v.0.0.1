# SolanaGo Tested Versions

Below is a comprehensive list of tested environment configurations for SolanaGo, verified by independent contributors.

## Core Dependencies

### Blockchain Requirements
- Solana CLI: 1.14.17 or later
- Anchor Framework: 0.27.0 or later
- Web3.js: 1.73.0 or later

### AI Model Requirements
- CUDA: 11.8, 12.0
- cuDNN: 8.6, 8.7
- TensorRT: 8.4, 8.5 (for GPU acceleration)

### Development Tools
- Rust: 1.69.0 or later
- Node.js: 16.x, 18.x
- Python: 3.8 or later (for tools and scripts)

## Verified Configurations

### Production Environment
```yaml
recommended:
  os: Ubuntu 22.04 LTS
  cuda: 11.8
  cudnn: 8.6
  tensorrt: 8.4
  solana_cli: 1.14.17
  anchor: 0.27.0
  rust: 1.69.0
```

### Development Environment
```yaml
minimum:
  os: Ubuntu 20.04 LTS
  cuda: 11.6
  cudnn: 8.4
  solana_cli: 1.13.0
  anchor: 0.26.0
  rust: 1.68.0
```

## GPU Compatibility

### NVIDIA GPUs
| GPU Series | Minimum Driver | Recommended Driver |
|------------|---------------|-------------------|
| RTX 40 Series | 525.60.13 | 535.104.05 |
| RTX 30 Series | 515.43.04 | 535.104.05 |
| RTX 20 Series | 515.43.04 | 535.104.05 |

### Compute Capabilities
- Minimum: 7.0
- Recommended: 8.6+
- Optimal: 8.9 (Ada Lovelace)

## Network Configurations

### RPC Nodes
- Mainnet-beta
- Devnet
- Testnet
- Local Validator

### Recommended RPC Providers
- Custom deployed RPC
- GenesysGo
- Helius
- QuickNode

## Operating Systems

### Linux (Primary Support)
- Ubuntu 20.04 LTS ✅
- Ubuntu 22.04 LTS ✅
- Debian 11 ✅

### Windows (Limited Support)
- Windows Server 2019 ⚠️
- Windows Server 2022 ⚠️
- Windows 10/11 Pro ⚠️

### macOS (CPU Only)
- M1/M2 Macs ⚠️ (CPU inference only)
- Intel Macs ⚠️ (CPU inference only)

## Known Issues

### CUDA Related
1. TensorRT 8.6+ requires CUDA 12.0
   - Solution: Stay on TensorRT 8.4 with CUDA 11.8

2. cuDNN 8.8+ compatibility issues
   - Solution: Use cuDNN 8.6 or 8.7

### Blockchain Related
1. Anchor Program versioning
   - Required: Use Anchor 0.27.0+ for IDL compatibility
   - Solution: Update Anchor toolchain

2. RPC node connection stability
   - Required: Implement proper retry logic
   - Solution: Use dedicated RPC nodes

## Performance Considerations

### GPU Settings
```yaml
optimal:
  batch_size: 64
  threads: 32
  compute_mode: "ExclusiveProcess"
  memory_allocation: "MaxMemory"
```

### Network Settings
```yaml
recommended:
  commitment: "confirmed"
  preflight_commitment: "processed"
  confirmation_blocks: 32
```

## Testing Tools
- Solana Test Validator
- Anchor Test Framework
- GPU Stress Test Suite
- Network Latency Monitor

## Contributors
Please submit your tested configurations via pull requests to our repository.

For the latest updates and detailed setup instructions, visit our [GitHub repository](https://github.com/your-org/solana-go).

Note: ✅ = Fully Supported, ⚠️ = Limited Support
