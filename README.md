# SharPi

## Proposed Architecture

```
┌─────────┬──────┬────────────────────────┬──────────────────────┐
│   CLI   │  TUI │  MCP Server Interface  │                      |
├─────────┴──────┴────────────────────────┘                      │
│                         API LAYER                              │
├─────────────────────────────────────┬──────────────────────────┤
│      Plugin Management              │ Core Context Managment   │
│  (Registration, Discovery,          │                          │
│       Configuration)                │ • Code Operations        │
├───────────┬─────────┬─────────────┬─┤ • Diff Generation        │
│           │         │             │ │ • Project Management     │
│  Client   │ Command │  Tools      │ │ • Conversation History   │
│  Plugins  │ Plugins │  Plugins    │ │ • MCP Services           │
│           │         │             │ │ • Prompt Management      │
│ ┌───────┐ │ ┌─────┐ │ ┌─────────┐ │ │ • Memory Management      │
│ │Claude │ │ │/add │ │ │Shell    │ │ │                          │
│ │OpenAI │ │ │@foo │ │ │Git      │ │ │                          │
│ │MCP    │ │ │!run │ │ │Editor   │ │ │                          │
│ │Custom │ │ │$vars│ │ │Files    │ │ │                          │
│ └───────┘ │ └─────┘ │ └─────────┘ │ │                          │
└───────────┴─────────┴─────────────┴─┴──────────────────────────┘
```

## Components

### Presentation Layer
- **CLI Interface**: Command-line user interaction
- **MCP Server Interface**: Programmatic access via MCP protocol

### API Layer
- **Context Management**: Application state and business logic, services
- **Plugin Management**: Registration and discovery of plugins

### Plugins
- **Client Plugins**: Claude, OpenAI, MCP-based, and custom implementations
- **Command Plugins**: User-accessible commands like /add, @foo, !run, $vars
- **Tools Plugins**: Shell, Git, Editor, File System operations
