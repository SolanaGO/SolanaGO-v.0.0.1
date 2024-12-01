# SolanaGo Game Review Partner (SGRP)

SGRP is a specialized analysis tool designed for SolanaGo that analyzes and reviews game records stored on the Solana blockchain.

## Key Features

- On-chain game data extraction and analysis
- AI model evaluation and suggestions
- Batch SGF file processing
- Real-time blockchain data synchronization
- DAO community review integration

## Configuration Files

Three optimized configuration templates are provided:
- GPU with TensorRT (Linux only): [config_gpu_tensorrt.yaml](/etc/config_gpu_tensorrt.yaml)
- GPU without TensorRT (Linux, Windows): [config_gpu.yaml](/etc/config_gpu.yaml)
- CPU only (all platforms): [config_cpu.yaml](/etc/config_cpu.yaml)

## Blockchain Data Analysis Configuration

In config.yaml:

```yaml
blockchain:
  # RPC Node Settings
  rpc_url: "https://api.mainnet-beta.solana.com"
  ws_url: "wss://api.mainnet-beta.solana.com"
  
  # Program IDs
  program_id: "<YOUR_PROGRAM_ID>"
  
  # Account Configuration  
  state_account: "<STATE_ACCOUNT>"
  history_account: "<HISTORY_ACCOUNT>"

  # Sync Settings
  sync_interval_ms: 1000
  max_history_fetch: 1000
```

## AI Analysis Configuration

```yaml
model:
  # Disable Time Settings
  timeout_ms: 0
  early_stop: false
  background_search: false
  
  # Fixed Computation Settings
  simulations_per_move: 3000
  
  # Debug Settings
  debugger:
    print_tree_depth: 20 
    print_tree_width: 3
```

## Usage Examples

1. Analyze Single Game:
```bash
sgrp analyze <GAME_ADDRESS> --verbose
```

2. Batch Analysis:
```bash 
sgrp batch-analyze games.txt --output analysis/
```

3. Real-time Game Analysis:
```bash
sgrp live-analyze <GAME_ADDRESS>
```

4. Export Analysis Report:
```bash
sgrp export-report <GAME_ADDRESS> --format pdf
```

## Integration Features

### 1. Blockchain Data Synchronization
```typescript
// Set up blockchain data listener
const dataSubscriber = new StateSubscriber(config.rpc_url);
dataSubscriber.subscribe(gameAddress, (update) => {
    // Process state updates
    processGameUpdate(update);
});
```

### 2. DAO Review Integration
```typescript
// Add community review
const review = {
    gameAddress,
    moveNumber: 50,
    content: "Interesting move choice...",
    author: reviewerPubkey,
};

// Publish to blockchain
await program.addReview(review);
```

### 3. AI Analysis Engine
```typescript
// Initialize analysis engine
const engine = new AnalysisEngine(config);

// Get position evaluation
const eval = await engine.evaluatePosition(gameState);
console.log(`Current position winrate: ${eval.winrate * 100}%`);

// Get move suggestions
const suggestions = await engine.getSuggestions(gameState, 3);
suggestions.forEach((sug, i) => {
    console.log(`Suggestion ${i+1}: ${sug.move} (winrate: ${sug.winrate * 100}%)`);
});
```

## Analysis Report Structure

Reports include:
- Game metadata (timestamp, players, result)
- Critical turning points analysis
- AI evaluation graphs
- Key mistakes identification
- Community reviews summary
- Improvement suggestions

## Performance Optimization

### 1. Memory Management
```typescript
const optimizeMemory = {
    cacheSize: '2GB',
    maxBatchSize: 16,
    pruneThreshold: 0.95,
};

const engine = new AnalysisEngine({
    ...config,
    optimization: optimizeMemory
});
```

### 2. Network Optimization
```typescript
const networkConfig = {
    maxRetries: 3,
    timeout: 5000,
    batchInterval: 100,
};

const subscriber = new StateSubscriber(rpcUrl, networkConfig);
```

## Important Notes

1. System Requirements:
- GPU Analysis: Minimum 8GB VRAM
- CPU Mode: Minimum 16GB RAM
- Storage: 20GB for model and cache

2. Network Requirements:
- Stable RPC connection
- Recommended: Private RPC node
- Websocket support for real-time updates

3. Security Considerations:
- Secure keypair storage
- Rate limiting for RPC calls
- Input validation for all transactions

## Error Handling

Common issues and solutions:
1. RPC Connection Errors
```typescript
try {
    await subscriber.connect();
} catch (e) {
    if (e.code === 'RATE_LIMITED') {
        await sleep(1000);
        retry();
    }
}
```

2. Transaction Failures
```typescript
try {
    await program.submitAnalysis(analysis);
} catch (e) {
    if (e.code === 'INVALID_BLOCKHASH') {
        await program.getRecentBlockhash();
        retry();
    }
}
```

## Community Integration

1. DAO Voting on Analysis Quality
```typescript
const analysisProposal = {
    analysisId: "...",
    quality: 85,
    stake: 100, // SOGO tokens
};

await daoProgram.submitVote(analysisProposal);
```

2. Rewards Distribution
```typescript
const rewardConfig = {
    baseReward: 10, // SOGO tokens
    qualityMultiplier: 1.5,
    stakingBonus: 1.2,
};

await program.distributeRewards(analysisId, rewardConfig);
```

For detailed information, visit our [GitHub repository](https://github.com/your-org/sgrp) or join our [Discord community](https://discord.gg/your-server).
