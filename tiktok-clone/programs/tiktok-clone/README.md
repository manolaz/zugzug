# TikTok Clone on Solana

A decentralized short-form video platform built on the Solana blockchain. This project demonstrates how to create a social media application with key TikTok features using Solana's high-performance blockchain.

## Overview

This program allows users to:
- Create and manage video content on-chain
- Like and comment on videos
- Follow other creators
- Moderate content through community governance

## Improvements in the 2025 Standard

### Modern Error Handling:
- Replaced ProgramResult with Result<()> return types
- Used Anchor's #[error_code] attribute for better error handling
- Added more specific error variants with descriptive messages

### Enhanced Security:
- Added more validation using require!() macros
- Improved authorization checks

### Code Quality:
- Used more idiomatic Rust patterns
- Simplified context structures
- Added comments for better readability

### Events System:
- Added events for important actions (create, like, moderate)
- Improved indexing and off-chain tracking capabilities

### Updated Dependencies:
- Added modern versions of anchor and solana dependencies
- Used the latest Rust edition

### Improved Resource Management:
- Better account space allocation
- More efficient data structures

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/tiktok-clone.git
cd tiktok-clone

# Install dependencies
npm install

# Build the program
anchor build
```

## Usage

1. Start a local Solana validator:
```bash
solana-test-validator
```

2. Deploy the program:
```bash
anchor deploy
```

3. Run the client:
```bash
npm run start
```

## Project Structure

```
tiktok-clone/
├── programs/
│   └── tiktok-clone/        # On-chain Solana program code
│       ├── src/             # Program source code
│       └── Cargo.toml       # Rust dependencies
├── app/                     # Frontend application
├── tests/                   # Integration tests
└── Anchor.toml              # Anchor configuration
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.