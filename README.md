# SharPi

## Architecture (PROPOSED, WIP)

```
+-------------+  +------------+  +------------+  +------------+
|   SPI CLI   |  |    TUI     |  |   NEOVIM   |  | DIRECT API |
| (IMPLEMENT) |  |  (EXAMPLE) |  |  (EXAMPLE) |  | (libsharpi)|
+------+------+  +-----+------+  +-----+------+  +-----+------+
       |               |               |               |
       |               |               |               |
       +-------+-------+---------------+               |
               |                                       |
               |                                       |
               v                                       v
   +-------------------------------------------+---------------+
   |            SOCKET API (daemon)            |     API       |
   |      (For Frontend Clients)               |   (Library)   |
   +-------------------------------------------+               |
   |                                                           |
   +-----------------------------------------------------------+
   |                      CORE SERVICES                        |
   |                                                           |
   |  +----------------------------------+  +---------------+  |
   |  |                                  |  |               |  |
   |  |        Plugin Management         |  | Context Mgmt  |  |
   |  |                                  |  |               |  |
   |  +----------------------------------+  +---------------+  |
   |                                                           |
   |  +-----------+-----------+-----------+  +---------------+ |
   |  |           |           |           |  |               | |
   |  |  Client   |  Command  |   Tools   |  | * Code Ops    | |
   |  |  Plugins  |  Plugins  |  Plugins  |  | * Diff Gen    | |
   |  |           |           |           |  | * Project Mgmt| |
   |  | +-------+ | +-------+ | +-------+ |  | * History     | |
   |  | |Claude | | |/add   | | |Shell  | |  | * Prompt Mgmt | |
   |  | |OpenAI | | |@foo   | | |Git    | |  | * Memory Mgmt | |
   |  | |MCP    | | |!run   | | |Editor | |  |               | |
   |  | |Custom | | |$vars  | | |Files  | |  |               | |
   |  | +-------+ | +-------+ | +-------+ |  |               | |
   |  |           |           |           |  |               | |
   |  +-----------+-----------+-----------+  +---------------+ |
   +-----------------------------------------------------------+
```

## Components

### Frontends
- **SPI CLI Frontend**: Command-line interface with subcommands like `init`, `chat`, `help`, `i`, and `daemon`
- **TUI**: Terminal UI for interactive use (example)
- **Neovim Plugin**: Direct integration with Neovim (example)
- **Direct API (libsharpi)**: Library for direct programmatic access to SharPi functionality

### SPI Daemon
- **API Layer**: Core interface to SharPi functionality
- **Plugin Management**: Registration, discovery, and configuration of plugins
- **Core Context Management**: Code operations, diff generation, project management, conversation history, prompt and memory management

### Plugins
- **Client Plugins**: Claude, OpenAI, MCP-based, and custom implementations
- **Command Plugins**: User-accessible commands like /add, @foo, !run, $vars
- **Tools Plugins**: Shell, Git, Editor, File System operations

## Command Structure

```bash
# Frontend commands that communicate with the daemon
spi init [--force]      # Initialize/reset configuration
spi chat -m "message"   # Send chat message to AI
spi --help              # Show help documentation
spi -i                  # Enter interactive mode

# Daemon management
spi daemon --start      # Start the daemon process
spi daemon --stop       # Stop the daemon
spi daemon --status     # Check daemon status
```

## Configuration

SharPi uses a TOML configuration file located at `~/.sharpi/config.toml`:

```toml
[clients]
default = "openai"

[clients.openai]
api_key = "your-api-key-here"
api_url = "https://api.openai.com/v1"
model = "gpt-4-turbo"
max_tokens = 1000
temperature = 0.7

[daemon]
port = 8080
auto_start = false
log_level = "info"

[interactive]
history_size = 100
prompt = "pi> "
```
