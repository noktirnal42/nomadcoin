# Mainnet Node Instructions

## Starting a Mainnet Node

1. **Bootstrap Node** (already running):
   ```
   # The bootstrap node is running on port 9333
   # Connect to it using: /ip4/127.0.0.1/tcp/9333/p2p/e076c356ba973b88
   ```

2. **Start Your Node**:
   ```bash
   # Using CLI
   ./nomadcoin-macos-cli node --port 9334 --bootstrap /ip4/127.0.0.1/tcp/9333/p2p/e076c356ba973b88

   # Using GUI - it will auto-detect mainnet when ./mainnet/node1/chaindata exists
   ```

3. **Mainnet Detection**:
   The GUI automatically detects mainnet when the directory `./mainnet/node1/chaindata` exists and configures:
   - Bootstrap peer: `/ip4/127.0.0.1/tcp/9333/p2p/e076c356ba973b88`
   - Mainnet flag: enabled

4. **Mining on Mainnet**:
   ```bash
   # CPU mining (1.0x boost)
   ./nomadcoin-macos-cli mine --address <your_address> --continuous

   # Mobile mining gets 1.5x boost automatically
   ```

## Important Notes

- **Do not commit mainnet data**: The `./mainnet/` directory contains real blockchain data and should NOT be committed to git
- **Backup your wallet**: Always backup your wallet.dat file
- **Network ports**: Default P2P port is 9333, RPC port is 9334
- **Block explorer**: Coming soon to https://explorer.nomadcoin.org

## Wallet Commands

```bash
# Create new wallet
./nomadcoin-macos-cli wallet

# Check balance
./nomadcoin-macos-cli status

# Send NOMAD
./nomadcoin-macos-cli send --to <address> --amount <amount> --memo <optional_memo>
```
