#!/usr/bin/env python3
"""
NomadCoin Wallet Import Utility
Allows importing existing wallet addresses using private keys
"""

import sys
import os

# Add the src directory to the path so we can import nomadcoin modules
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

def import_wallet_from_private_key(private_key_hex):
    """Import an address from a private key hex string"""
    try:
        # Import the necessary modules
        from wallet import Wallet, WalletAddress
        from crypto import generate_address
        from ed25519_dalek import SigningKey
        
        # Validate and decode private key
        private_key_bytes = bytes.fromhex(private_key_hex)
        
        # Derive public key from private key
        signing_key = SigningKey.from_bytes(private_key_bytes)
        verifying_key = signing_key.verifying_key()
        
        public_key_hex = verifying_key.to_bytes().hex()
        address = generate_address(public_key_hex)
        
        # Create wallet address object
        wallet_address = WalletAddress(
            public_key=public_key_hex,
            private_key=private_key_hex,
            address=address
        )
        
        return wallet_address
    except Exception as e:
        print(f"Error importing private key: {e}")
        return None

def main():
    print("🔐 NomadCoin Wallet Import Utility")
    print("==================================")
    
    if len(sys.argv) > 1:
        # Private key provided as argument
        private_key = sys.argv[1]
    else:
        # Ask for private key
        private_key = input("Enter your private key (hex): ").strip()
    
    if not private_key:
        print("❌ No private key provided")
        return 1
        
    # Validate hex format
    try:
        bytes.fromhex(private_key)
        if len(private_key) != 64:  # 32 bytes = 64 hex chars
            print("⚠️  Warning: Private key should be 64 hex characters (32 bytes)")
    except ValueError:
        print("❌ Invalid hex format for private key")
        return 1
    
    # Import the address
    wallet_address = import_wallet_from_private_key(private_key)
    
    if wallet_address is None:
        return 1
        
    print("\n✅ Successfully imported address!")
    print(f"Address:   {wallet_address.address}")
    print(f"Public Key: {wallet_address.public_key}")
    print(f"Private Key: {wallet_address.private_key} (KEEP SECRET!)")
    
    # Show how to use it
    print("\n💰 To use this address for mining:")
    print(f"./releases/nomadcoin-macos-cli mine --address {wallet_address.address} --continuous")
    
    print("\n💾 To save this wallet for future use:")
    print("(The wallet.dat file will be created automatically when you first use the address)")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
