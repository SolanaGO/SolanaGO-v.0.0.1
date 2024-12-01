# SolanaGo Frequently Asked Questions

## General Questions

### Blockchain & Network

#### Q1: How do I connect to different Solana networks?
You can connect to different networks by modifying the network parameter:
```bash
# For devnet
--network devnet

# For testnet  
--network testnet

# For mainnet
--network mainnet-beta
```

#### Q2: What is the cost per game?
Transaction costs vary but typically:
- Move validation: ~0.000005 SOL
- Game state update: ~0.000003 SOL
- AI model verification: ~0.000008 SOL
Average total cost per game: ~0.001 SOL

#### Q3: How are game states stored on-chain?
Game states are stored in a program-derived account (PDA) with the following structure:
```rust
pub struct GameState {
    pub current_board: [u8; 361],
    pub moves_history: Vec<Move>,
    pub last_timestamp: i64,
    pub black_player: Pubkey,
    pub white_player: Pubkey,
    pub current_turn: u8,
    pub game_status: GameStatus,
}
```

### AI & Model

#### Q4: Where can I find win rate predictions?
Win rates are logged in two places:
1. On-chain state (for verification)
2. Local logs with format:
```
move(b): dp, winrate=44.110905%, N=654, Q=-0.117782, p=0.079232, v=-0.116534
```

#### Q5: How do I view analysis variations?
Add `--verbose` flag to see detailed analysis:
```bash
solana-go --verbose
```
Output includes:
- Main variation path
- Secondary variations
- Position evaluation
- Model confidence

### Setup & Configuration

#### Q6: How do I setup with hardware acceleration?

For GPU Support:
```bash
# Install dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.14.17/install)"

# Configure GPU
solana-go configure --cuda --tensor-cores
```

For CPU-only:
```bash
solana-go configure --cpu-only
```

#### Q7: How do I adjust thinking time?

Modify the configuration file:
```yaml
time_settings:
  base_time_ms: 5000  # 5 seconds base time
  increment_ms: 1000  # 1 second increment
  max_move_time: 30000 # 30 seconds max
```

### Integration & Development

#### Q8: How do I integrate with existing Solana dApps?

1. Import the SDK:
```typescript
import { SolanaGo } from '@solana-go/sdk'
```

2. Initialize connection:
```typescript
const game = new SolanaGo({
  network: 'devnet',
  commitment: 'confirmed'
})
```

3. Make moves:
```typescript
await game.makeMove({
  position: {x: 3, y: 4},
  signature: playerKeypair
})
```

#### Q9: How do I participate in governance?

1. Hold SOGO tokens
2. Stake them in governance contract
3. Create/vote on proposals through:
```bash
solana-go governance propose --title "Improve Model V2"
```

## Advanced Topics

### Performance Optimization

#### Q10: How to optimize transaction performance?
- Use versioned transactions
- Implement compute budget instructions
- Batch similar operations

Example:
```typescript
const tx = new VersionedTransaction(
  new TransactionMessage({
    instructions: [
      ComputeBudgetProgram.setComputeUnitLimit({ units: 1_400_000 }),
      ... // game instructions
    ],
    ... // other transaction details
  })
)
```

### Model & AI Specifics

#### Q11: How to customize model parameters?
Edit the model config:
```yaml
model_config:
  batch_size: 16
  search_threads: 8
  eval_threads: 4
  tree_size: "2000M"
```

### Error Resolution

#### Q12: Common Error: Transaction Simulation Failed
Usually caused by:
1. Insufficient SOL balance
2. Invalid game state transition
3. Rate limiting

Solution:
```bash
# Check account balance
solana balance

# Verify game state
solana-go verify-state --game <GAME_ID>
```

### Security Considerations

#### Q13: How is move validation secured?
- Moves are validated by consensus
- Model outputs are verified on-chain
- State transitions require signed transactions

Example validation:
```rust
pub fn validate_move(ctx: Context<ValidateMove>) -> Result<()> {
    require!(ctx.accounts.game.is_active(), ErrorCode::GameNotActive);
    require_keys_eq!(
        ctx.accounts.player.key(),
        ctx.accounts.game.current_player,
        ErrorCode::WrongPlayer
    );
    // ... additional validation
}
```

## Development Tools

### Q14: Recommended Testing Tools
- Solana Test Validator
- Anchor Testing Framework
- Model Evaluation Tools

Example test setup:
```typescript
describe('SolanaGo', () => {
  const program = anchor.workspace.SolanaGo;
  
  it('Initializes game correctly', async () => {
    const [gameAccount] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from('game'), player.publicKey.toBuffer()],
      program.programId
    );
    // ... test implementation
  });
});
```

### Q15: Development Environment Setup
```bash
# Install development dependencies
npm install -g @project-serum/anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# Setup local validator
solana-test-validator

# Deploy program
anchor deploy
```

For more technical details, visit our [GitHub repository](https://github.com/your-org/solana-go).
