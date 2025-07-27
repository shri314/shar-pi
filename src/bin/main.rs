// Copyright (c) 2025 SharPi Contributors
// MIT License

use sharpi::clients::openai;
use sharpi::config;
use anyhow::{anyhow, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let command = args.get(1).cloned();

    match command.as_deref() {
        Some("init") => {
            let subcommand = args.get(2).cloned();

            match subcommand.as_deref() {
                Some("--force") => {
                    config::create_default_config(true)
                },
                Some("help") => {
                    println!("SharPi Init - Configuration Management");
                    println!("");
                    println!("USAGE:");
                    println!("  spi init [OPTIONS]");
                    println!("");
                    println!("OPTIONS:");
                    println!("  --force                   Reinitialize, overwriting existing configuration");
                    println!("  help                      Show this help message");
                    Ok(())
                },
                None => {
                    // Default behavior - initialize without force
                    config::create_default_config(false)
                },
                _ => {
                    eprintln!("Unknown init command. Use: spi init help for usage information");
                    Ok(())
                }
            }
        },

        Some("chat") => {
            let subcommand = args.get(2).cloned();

            match subcommand.as_deref() {
                // List conversations
                Some("ls") | Some("list") => {
                    match sharpi::core::history::load_history() {
                        Ok(history) => {
                            match history.list_conversations() {
                                Ok(conversations) => {
                                    if conversations.is_empty() {
                                        println!("No conversations found.");
                                    } else {
                                        println!("Conversations:");
                                        for (id, metadata) in &conversations {
                                            let active_marker = if Some(id.clone()) == history.active_conversation_id {
                                                "* "
                                            } else {
                                                "  "
                                            };

                                            let message_count = metadata.message_count;
                                            let last_updated = metadata.updated_at.format("%Y-%m-%d %H:%M");

                                            println!("{}{} - {} ({}messages, updated: {})",
                                                active_marker,
                                                id,
                                                metadata.title,
                                                message_count,
                                                last_updated
                                            );
                                        }
                                    }
                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("Error listing conversations: {}", err);
                                    Err(err)
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("Error loading conversation history: {}", err);
                            Err(err)
                        }
                    }
                },

                // Create new conversation
                Some("new") => {
                    let mut title_index = None;

                    // Check for -t flag
                    for i in 3..args.len() {
                        if args[i] == "-t" && i + 1 < args.len() {
                            title_index = Some(i + 1);
                            break;
                        }
                    }

                    if title_index.is_none() {
                        return Err(anyhow!("Usage: spi chat new -t \"Conversation Title\""));
                    }

                    let title = args[title_index.unwrap()].clone();

                    match sharpi::core::history::load_history() {
                        Ok(mut history) => {
                            match history.create_conversation(title) {
                                Ok((id, conversation)) => {
                                    println!("Created new conversation: {} (ID: {})", conversation.title, id);
                                    sharpi::core::history::save_history(&history)?;
                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("Error creating conversation: {}", err);
                                    Err(err)
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("Error loading history: {}", err);
                            Err(err)
                        }
                    }
                },

                // Show conversation
                Some("show") => {
                    let conversation_id_option = if args.len() > 3 {
                        Some(args[3].clone())
                    } else {
                        None
                    };

                    match sharpi::core::history::load_history() {
                        Ok(history) => {
                            // Use active conversation if no ID provided
                            let conversation_id = match conversation_id_option {
                                Some(id) => id,
                                None => match &history.active_conversation_id {
                                    Some(id) => id.clone(),
                                    None => return Err(anyhow!("No active conversation. Use: spi chat show <conversation_id>"))
                                }
                            };

                            match history.get_conversation(&conversation_id) {
                                Ok(conversation) => {
                                    println!("Conversation: {} (ID: {})", conversation.title, conversation_id);
                                    println!("Created: {}", conversation.created_at.format("%Y-%m-%d %H:%M"));
                                    println!("Messages: {}", conversation.messages.len());
                                    println!();

                                    if conversation.messages.is_empty() {
                                        println!("No messages in this conversation.");
                                    } else {
                                        for (i, message) in conversation.messages.iter().enumerate() {
                                            let role = if message.role == "user" { "You" } else { "AI" };
                                            let timestamp = message.timestamp.format("%Y-%m-%d %H:%M");
                                            println!("[{}] {}: {}", timestamp, role, message.content);

                                            if i < conversation.messages.len() - 1 && message.role == "assistant" {
                                                println!();
                                            }
                                        }
                                    }

                                    Ok(())
                                },
                                Err(_) => {
                                    eprintln!("Conversation with ID '{}' not found.", conversation_id);
                                    Err(anyhow!("Conversation not found"))
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("Error loading conversation history: {}", err);
                            Err(err)
                        }
                    }
                },

                // Send message
                Some("send") => {
                    let mut conversation_id = None;
                    let mut message_index = None;

                    // Check if the third argument is an ID (not starting with a dash)
                    if args.len() > 3 && !args[3].starts_with('-') {
                        conversation_id = Some(args[3].clone());
                    }

                    // Process all arguments for -m flag
                    for i in 3..args.len() {
                        if args[i] == "-m" && i + 1 < args.len() {
                            message_index = Some(i + 1);
                        }
                    }

                    if message_index.is_none() {
                        return Err(anyhow!("Usage: spi chat send -m \"your message\" or spi chat send <conversation_id> -m \"your message\""));
                    }

                    let message = args[message_index.unwrap()].clone();

                    println!("SharPi - AI Coding Assistant");
                    println!("Sending request to AI API with conversation history...");

                    match openai::call_openai_with_history(&message, conversation_id.as_deref(), None) {
                        Ok(response) => {
                            println!("\nResponse from AI API:");
                            println!("{}", response);
                            Ok(())
                        },
                        Err(err) => {
                            eprintln!("Error calling AI API: {}", err);
                            eprintln!("\nMake sure your configuration is set up correctly:");
                            eprintln!("Run 'spi init' to create a default configuration file");
                            eprintln!("Then edit ~/.sharpi/config.toml with your API keys");
                            Err(err)
                        }
                    }
                },

                // Remove conversation
                Some("rm") => {
                    if args.len() <= 3 {
                        return Err(anyhow!("Usage: spi chat rm <conversation_id>"));
                    }

                    let conversation_id = args[3].clone();

                    match sharpi::core::history::load_history() {
                        Ok(mut history) => {
                            match history.remove_conversation(&conversation_id) {
                                Ok(()) => {
                                    println!("Removed conversation with ID: {}", conversation_id);
                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("Error removing conversation: {}", err);
                                    Err(err)
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("Error loading history: {}", err);
                            Err(err)
                        }
                    }
                },

                // Set active conversation
                Some("use") => {
                    if args.len() <= 3 {
                        return Err(anyhow!("Usage: spi chat use <conversation_id>"));
                    }

                    let conversation_id = args[3].clone();

                    match sharpi::core::history::load_history() {
                        Ok(mut history) => {
                            match history.set_active_conversation(conversation_id.clone()) {
                                Ok(true) => {
                                    println!("Set active conversation to ID: {}", conversation_id);
                                    sharpi::core::history::save_history(&history)?;
                                    Ok(())
                                },
                                Ok(false) => {
                                    eprintln!("Conversation with ID '{}' not found.", conversation_id);
                                    Err(anyhow!("Conversation not found"))
                                },
                                Err(err) => {
                                    eprintln!("Error setting active conversation: {}", err);
                                    Err(err)
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("Error loading history: {}", err);
                            Err(err)
                        }
                    }
                },

                // Chat help
                Some("help") => {
                    print_chat_help();
                    Ok(())
                },

                // No subcommand provided
                None => {
                    print_chat_help();
                    Ok(())
                },

                _ => {
                    print_chat_help();
                    Ok(())
                }
            }
        },

        Some("daemon") => {
            let subcommand = args.get(2).cloned();

            match subcommand.as_deref() {
                Some("start") => {
                    println!("Starting SharPi daemon... (Not implemented yet)");
                    Ok(())
                },
                Some("stop") => {
                    println!("Stopping SharPi daemon... (Not implemented yet)");
                    Ok(())
                },
                Some("status") => {
                    println!("SharPi daemon is not running (Not implemented yet)");
                    Ok(())
                },
                Some("help") => {
                    println!("SharPi Daemon - Background Service Management");
                    println!("");
                    println!("USAGE:");
                    println!("  spi daemon COMMAND");
                    println!("");
                    println!("COMMANDS:");
                    println!("  start                     Start the SharPi daemon");
                    println!("  stop                      Stop the SharPi daemon");
                    println!("  status                    Check daemon status");
                    println!("  help                      Show this help message");
                    Ok(())
                },
                _ => {
                    eprintln!("Unknown daemon command. Use: spi daemon help for usage information");
                    Ok(())
                }
            }
        },

        Some("-i") => {
            println!("Interactive mode not implemented yet.");
            println!("In the future, you'll see a prompt like:");
            println!("pi> ");
            Ok(())
        },

        Some("--help") | Some("-h") | None => {
            print_help(&program);
            Ok(())
        },

        Some(_) => {
            eprintln!("Unknown command. Use 'spi --help' for usage information.");
            print_help(&program);
            Ok(())
        }
    }
}

fn print_chat_help() {
    println!("SharPi Chat - Conversation Management");
    println!("");
    println!("USAGE:");
    println!("  spi chat COMMAND [OPTIONS]");
    println!("");
    println!("COMMANDS:");
    println!("  send -m \"message\"         Send a message to the active conversation");
    println!("  send <id> -m \"msg\"        Send a message in specific conversation");
    println!("  ls                        List all conversations (alias: list)");
    println!("  new -t \"title\"            Create a new conversation");
    println!("  show                      Show active conversation details");
    println!("  show <id>                 Show specific conversation details");
    println!("  rm <id>                   Remove a conversation");
    println!("  use <id>                  Set as active conversation");
    println!("  help                      Show this help message");
}

fn print_help(program: &str) {
    println!("SharPi - AI Coding Assistant");
    println!("");
    println!("USAGE:");
    println!("  {} COMMAND [OPTIONS]", program);
    println!("");
    println!("COMMANDS:");
    println!("  init [command]            Configuration management (run 'spi init help')");
    println!("  chat [command]            Conversation management (run 'spi chat help')");
    println!("  -i                        Enter interactive mode");
    println!("  daemon [command]          Daemon management (run 'spi daemon help')");
    println!("  --help, -h                Show this help message");
}
