# 📖 Flarecord - Discord Bot Framework for Cloudflare Workers

Welcome to **Flarecord** - A lightweight, type-safe, asynchronous framework for building Discord bots on **Cloudflare Workers** using Rust and WebAssembly.

> **Flarecord** = **Flare** (Cloudflare) + **cord** (Discord)

---

## 📚 Documentation

### Getting Started
- **[Quick Start Guide](./GETTING_STARTED.md)** - Set up your first bot in minutes
- **[Installation & Setup](./GETTING_STARTED.md#installation)** - Dependencies and environment configuration

### API Reference
Organized by feature, each document covers a specific part of the framework:

- **[docs/COMMANDS.md](./docs/COMMANDS.md)** - Command trait, builders, and execution
- **[docs/OPTIONS.md](./docs/OPTIONS.md)** - Command parameters and option types
- **[docs/RESPONSES.md](./docs/RESPONSES.md)** - Response building and Discord messages
- **[docs/INTERACTIONS.md](./docs/INTERACTIONS.md)** - Interaction data and context
- **[docs/ERRORS.md](./docs/ERRORS.md)** - Error handling and types
- **[docs/BOT.md](./docs/BOT.md)** - Bot initialization and configuration

### Architecture & Design
- **[docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)** - Internal design and patterns
- **[docs/WASM.md](./docs/WASM.md)** - WebAssembly considerations

### Examples & Development
- **[Examples](./examples/)** - Working code examples
- **[CONTRIBUTING.md](./CONTRIBUTING.md)** - How to contribute
- **[TROUBLESHOOTING.md](./TROUBLESHOOTING.md)** - Common issues and solutions

---

## ✨ Key Features

| Feature | Status | Benefit |
|---------|--------|---------|
| **Slash Commands** | ✅ | Type-safe command handling |
| **Command Options** | ✅ | String, integer, user, channel, role, attachment options |
| **Responses** | ✅ | Text, embeds, ephemeral, deferred responses |
| **Interactions** | ✅ | Full interaction context and data access |
| **Error Handling** | ✅ | Comprehensive error types with `thiserror` |
| **WASM Optimized** | ✅ | Global distribution on Cloudflare Workers |
| **Type Safety** | ✅ | Compile-time guarantees for Discord interactions |
| **Message Components** | 🚧 | Buttons, dropdowns, modals (in development) |
| **Discord Gateway** | ❌ | Not supported (see Limitations) |

---

## 🎯 Use Cases

### ✅ Perfect For:
- Serverless Discord bots with global reach
- Command-based bots (slash commands)
- Low-latency, cost-effective deployments
- Type-safe Discord interaction handling

### ⚠️ Limitations:
- **No Discord Gateway**: Cannot listen to real-time guild events (member joins, messages, etc.)
  - **Future Solution**: Will support gateway events via [Cloudflare Durable Objects](https://developers.cloudflare.com/durable-objects/)
  - Only possibile if this issue gets fixed: [https://github.com/cloudflare/workerd/issues/486](https://github.com/cloudflare/workerd/issues/4864)
- No support for voice channels
- No support for presence updates
- Limited to interaction-based workflows

---

## 📋 Feature Roadmap

### Currently Implemented ✅
- Core slash command handling
- Command option types (string, integer, boolean, user, channel, role, attachment)
- Command response building
- Ephemeral and deferred responses
- Interaction context and data resolution
- Automatic command synchronization
- WASM/Cloudflare Workers support

### In Development 🚧
- Message components support (buttons, select menus, text inputs)
- Modal handling
- Auto-complete interactions
- Advanced embed builders

### Planned 📋
- Discord Gateway events via Durable Objects
- Database integration helpers
- Rate limiting utilities
- Advanced caching strategies
- Webhook support

### Discord API Coverage

Below is a checklist of Discord API features and their support status in Flarecord:

#### Interactions ✅
- [x] Slash commands
- [x] Command options (all types)
- [x] Message components (buttons, select menus)
- [x] Auto-complete suggestions
- [ ] Modal submissions
- [ ] Presence (REST endpoint)
- [ ] Activities (REST endpoint)

#### Responses ✅
- [x] Text responses
- [x] Embeds
- [x] Ephemeral responses
- [x] Deferred responses
- [ ] Follow-up messages (via webhooks)
- [x] Message attachments (partial)

#### Gateway Events ❌
Currently **not supported**. Future implementation planned using Cloudflare Durable Objects:
- [ ] READY
- [ ] MESSAGE_CREATE
- [ ] MESSAGE_UPDATE
- [ ] MESSAGE_DELETE
- [ ] GUILD_MEMBER_ADD
- [ ] GUILD_MEMBER_UPDATE
- [ ] GUILD_MEMBER_REMOVE
- [ ] PRESENCE_UPDATE
- [ ] VOICE_STATE_UPDATE
- [ ] And other gateway events...

#### Twilight Models Coverage
Flarecord uses the [Twilight](https://twilight.rs/) library for Discord types. Current coverage:

- [x] Command models (`CommandInteraction`, `CommandOption`, etc.)
- [x] User models (`User`, `Member`)
- [x] Channel models (`Channel`, `ChannelId`)
- [x] Guild models (`Guild`, `GuildId`)
- [x] Message models (`Message`, `Embed`)
- [ ] Voice models (not needed for webhooks)
- [ ] Presence models (gateway-only)
- [ ] Activity models (gateway-only)

---

## 🚀 Quick Start

### 1. Create a new bot
```bash
cargo new my-bot
cd my-bot
```

### 2. Add Flarecord to `Cargo.toml`
```toml
[dependencies]
flarecord = { git = "https://github.com/GitGinocchio/flarecord-rs" }
worker = "^0.8"
serde = { version = "^1.0", features = ["derive"] }
```

### 3. Create your first command
```rust
use flarecord::prelude::*;
use async_trait::async_trait;

pub struct HelloCommand;

#[async_trait(?Send)]
impl Command for HelloCommand {
    fn name(&self) -> String { "hello".into() }
    fn description(&self) -> String { "Say hello!".into() }
    fn options(&self) -> BotResult<CommandOptions> { Ok(None) }
    
    async fn execute(
        &self,
        _: CommandInteraction,
        _: CommandContext,
    ) -> BotResult<CommandResponse> {
        Ok(CommandResponseBuilder::new()
            .content("Hello! 👋")
            .build())
    }
}
```

### 4. Deploy to Cloudflare Workers
```bash
wrangler build --target wasm32-unknown-unknown --release
wrangler deploy
```

**Full guide**: [GETTING_STARTED.md](./GETTING_STARTED.md)

---

## 📖 Documentation Structure

```
docs/
├── COMMANDS.md         - Command trait & builders
├── OPTIONS.md          - Command parameters
├── RESPONSES.md        - Response building
├── INTERACTIONS.md     - Interaction context
├── ERRORS.md           - Error handling
├── BOT.md              - Bot configuration
├── ARCHITECTURE.md     - Internal design
└── WASM.md             - WebAssembly considerations
```

Each document is focused on a specific aspect of Flarecord, making it easy to find what you need.

---

## 🔗 Resources

- **GitHub**: [GitGinocchio/flarecord-rs](https://github.com/GitGinocchio/flarecord-rs)
- **Discord API Docs**: [discord.com/developers/docs](https://discord.com/developers/docs)
- **Cloudflare Workers**: [developers.cloudflare.com/workers](https://developers.cloudflare.com/workers)
- **Twilight**: [twilight.rs](https://twilight.rs/)
- **Rust Book**: [doc.rust-lang.org/book](https://doc.rust-lang.org/book)

---

## 📊 Project Status

**Current Version**: 0.1.0 (Alpha)

Flarecord is actively being developed. While core functionality is stable, expect breaking changes during the 0.x versions.

**Development Focus**:
- Message components and modals
- Improved documentation
- Gateway event support (planned)

---

## 🤝 Contributing

Interested in contributing? We welcome all contributions!

- Read [CONTRIBUTING.md](./CONTRIBUTING.md)
- Check [GitHub Issues](https://github.com/GitGinocchio/flarecord-rs/issues)
- Review [Architecture](./docs/ARCHITECTURE.md)

---

## 🙏 Credits

Inspired by [stateless-discord-bot](https://github.com/siketyan/stateless-discord-bot)

Built with:
- [Twilight](https://twilight.rs/) - Discord models and types
- [Cloudflare Workers](https://workers.cloudflare.com/) - Serverless platform

---

## 📄 License

See [LICENSE](./LICENSE) file for details.

---

**Ready to build?** Start with the [Quick Start Guide](./GETTING_STARTED.md) or explore the [Examples](./examples/). 🚀
