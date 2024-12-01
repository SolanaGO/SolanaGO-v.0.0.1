# SolanaGo CLI Reference

## Core Commands

### Network & Connection
```bash
--network <network>           # solana network (mainnet-beta, devnet, testnet)
--rpc-url <url>              # custom RPC endpoint
--ws-url <url>               # websocket endpoint
--commitment <commitment>     # confirmation commitment (processed, confirmed, finalized)
```

### Game Operations
```bash
--init-game                  # initialize new game
--make-move <position>       # make a move (e.g., "d4")
--resign                     # resign current game
--analyze                    # analyze current position
--time-settings <settings>   # set time controls
```

### AI Model Control
```bash
--model-path <path>          # path to AI model
--batch-size <size>          # inference batch size
--gpu-list <list>           # comma-separated GPU devices
--eval-threads <num>         # evaluation thread count
--search-threads <num>       # search thread count
```

### Blockchain Integration
```bash
--program-id <id>            # SolanaGo program ID
--payer <keypair>           # transaction payer keypair
--state-account <pubkey>     # game state account
--history-account <pubkey>   # move history account
```

## Advanced Options

### Performance Tuning
```bash
--tree-size <size>           # maximum search tree size
--cache-size <size>          # position cache size
--parallel-games <num>       # number of parallel games
--timeout-ms <ms>            # operation timeout
```

### Development & Debug
```bash
--verbose                    # enable verbose logging
--debug                     # enable debug mode
--test-validator            # use local test validator
--simulation               # simulation mode without transactions
```

### Security & Authorization
```bash
--auth-token <token>         # authorization token
--key-path <path>           # keypair file path
--hardware-wallet           # use hardware wallet
--require-confirmation      # require manual confirmation
```

## Example Usage

1. Initialize New Game
```bash
solana-go --network devnet --init-game --payer game_keypair.json
```

2. Make Move with Analysis
```bash
solana-go --network devnet \
          --make-move "d4" \
          --analyze \
          --verbose
```

3. Tournament Mode Setup
```bash
solana-go --network mainnet-beta \
          --batch-size 16 \
          --gpu-list 0,1 \
          --tree-size 4000M \
          --time-settings "60000:5000"
```

4. Development Testing
```bash
solana-go --network localhost \
          --test-validator \
          --debug \
          --verbose
```

## Environment Variables
```bash
# Network Configuration
SOLANA_GO_NETWORK=devnet
SOLANA_GO_RPC_URL=https://api.devnet.solana.com

# Authentication
SOLANA_GO_AUTH_TOKEN=xxx
SOLANA_GO_KEYPAIR_PATH=/path/to/keypair.json

# Performance
SOLANA_GO_BATCH_SIZE=16
SOLANA_GO_GPU_LIST=0,1

# Development
SOLANA_GO_DEBUG=true
SOLANA_GO_VERBOSE=true
```

## Configuration File Reference
The CLI can also read from a YAML configuration file:

```yaml
network:
  cluster: "devnet"
  rpc_url: "https://api.devnet.solana.com"

game:
  program_id: "<PROGRAM_ID>"
  state_account: "<PUBKEY>"

model:
  batch_size: 16
  gpu_list: "0,1"
  
security:
  require_confirmation: true
  hardware_wallet: true
```

Use with:
```bash
solana-go --config path/to/config.yaml
```

## Error Codes
- 1: Network connection error
- 2: Invalid move
- 3: Transaction error
- 4: Model loading error
- 5: Account error
- 6: Authorization error
- 7: Timeout error
- 8: State validation error
