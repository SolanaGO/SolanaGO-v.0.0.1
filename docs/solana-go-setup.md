# SolanaGo Environment Setup Guide

This guide demonstrates the setup process using Ubuntu 22.04 LTS with CUDA 11.8, cuDNN 8.6, and Solana CLI 1.14.17. While other configurations are possible, these versions have been thoroughly tested and verified.

## 1. Blockchain Environment Setup

### Configure Solana CLI

First, install the Solana CLI tools:
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.14.17/install)"
```

Add Solana to your path by editing `~/.bashrc`:
```bash
# Add Solana paths
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
export SOLANA_HOME="$HOME/.config/solana"
```

Verify installation:
```bash
solana --version
solana-keygen --version
```

### Configure Anchor Framework
```bash
# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest

# Add to path
export PATH="/home/$USER/.avm/bin:$PATH"
```

## 2. AI Environment Setup

### CUDA Installation

Install CUDA dependencies:
```bash
# Add NVIDIA package repositories
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-ubuntu2204.pin
sudo mv cuda-ubuntu2204.pin /etc/apt/preferences.d/cuda-repository-pin-600
wget https://developer.download.nvidia.com/compute/cuda/11.8.0/local_installers/cuda-repo-ubuntu2204-11-8-local_11.8.0-520.61.05-1_amd64.deb
sudo dpkg -i cuda-repo-ubuntu2204-11-8-local_11.8.0-520.61.05-1_amd64.deb
sudo cp /var/cuda-repo-ubuntu2204-11-8-local/cuda-*-keyring.gpg /usr/share/keyrings/

# Install CUDA
sudo apt-get update
sudo apt-get -y install cuda-11-8
```

Add CUDA to your path in `~/.bashrc`:
```bash
# CUDA paths
export PATH="/usr/local/cuda-11.8/bin:$PATH"
export LD_LIBRARY_PATH="/usr/local/cuda-11.8/lib64:$LD_LIBRARY_PATH"
```

### cuDNN Setup
```bash
# Install cuDNN
sudo apt-get install libcudnn8=8.6.*-1+cuda11.8
sudo apt-get install libcudnn8-dev=8.6.*-1+cuda11.8
```

Verify the installation:
```bash
# Check CUDA
nvcc --version

# Test cuDNN
cp -r /usr/src/cudnn_samples_v8/ ~/
cd ~/cudnn_samples_v8/mnistCUDNN
make clean && make
./mnistCUDNN
# Should display "Test passed!"
```

## 3. Development Environment

### Rust Setup
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to path
source $HOME/.cargo/env

# Add components
rustup component add rustfmt
rustup component add clippy
```

### Node.js Setup
```bash
# Install Node.js using nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install 16
nvm use 16
```

## 4. Location Verification

Check critical library locations:
```bash
# CUDA libraries
locate libcudart.so
locate libcudnn.so.8

# Expected output:
/usr/local/cuda-11.8/targets/x86_64-linux/lib/libcudart.so
/usr/local/cuda-11.8/targets/x86_64-linux/lib/libcudart.so.11.8
/usr/lib/x86_64-linux-gnu/libcudnn.so.8
/usr/lib/x86_64-linux-gnu/libcudnn.so.8.6.0
```

If libraries aren't found:
```bash
sudo updatedb && locate libcudart.so && locate libcudnn.so.8
```

## 5. Network Configuration

### RPC Setup
```bash
# Configure default RPC endpoint
solana config set --url https://api.devnet.solana.com

# Or for mainnet
solana config set --url https://api.mainnet-beta.solana.com
```

### Wallet Setup
```bash
# Generate new keypair
solana-keygen new --outfile ~/solana-go-wallet.json

# Set as default wallet
solana config set --keypair ~/solana-go-wallet.json
```

## 6. Security Considerations

### Secure Key Storage
```bash
# Set restrictive permissions
chmod 600 ~/solana-go-wallet.json

# Use hardware wallet
solana config set --keypair usb://ledger
```

### Environment Variables
```bash
# Add to ~/.bashrc
export SOLANA_GO_NETWORK="devnet"  # or mainnet-beta
export SOLANA_GO_RPC_TIMEOUT=60
export SOLANA_GO_COMMITMENT="confirmed"
```

## Common Issues and Solutions

### 1. CUDA Path Issues
If CUDA is not found:
```bash
export CUDA_HOME=/usr/local/cuda-11.8
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CUDA_HOME/lib64
```

### 2. Solana CLI Connection Issues
```bash
# Check connection
solana cluster-version

# If timeout occurs, try different RPC
solana config set --url https://your-custom-rpc.com
```

### 3. Permission Issues
```bash
# Fix library permissions
sudo ldconfig
sudo usermod -a -G video $USER
```

## Additional Resources

- [Solana Documentation](https://docs.solana.com)
- [CUDA Documentation](https://docs.nvidia.com/cuda)
- [cuDNN Documentation](https://docs.nvidia.com/deeplearning/cudnn)
- [Anchor Book](https://book.anchor-lang.com)

For project-specific issues, please refer to our [GitHub repository](https://github.com/your-org/solana-go) or join our [Discord community](https://discord.gg/your-server).
