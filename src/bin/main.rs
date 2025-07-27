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
            let force = args.iter().any(|arg| arg == "--force");
            config::create_default_config(force)
        },

        Some("chat") => {
            if args.len() < 3 || args[2] != "-m" {
                return Err(anyhow!("Usage: spi chat -m \"your message\""));
            }

            let message = if args.len() > 3 {
                args[3].clone()
            } else {
                return Err(anyhow!("No message provided. Usage: spi chat -m \"your message\""));
            };

            println!("SharPi - AI Coding Assistant");
            println!("Sending request to AI API...");

            match openai::call_openai(&message, None) {
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

        Some("daemon") => {
            let subcommand = args.get(2).cloned();

            match subcommand.as_deref() {
                Some("--start") => {
                    println!("Starting SharPi daemon... (Not implemented yet)");
                    Ok(())
                },
                Some("--stop") => {
                    println!("Stopping SharPi daemon... (Not implemented yet)");
                    Ok(())
                },
                Some("--status") => {
                    println!("SharPi daemon is not running (Not implemented yet)");
                    Ok(())
                },
                _ => {
                    eprintln!("Unknown daemon command. Use: spi daemon --start|--stop|--status");
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
            // Unknown command
            eprintln!("Unknown command. Use 'spi --help' for usage information.");
            print_help(&program);
            Ok(())
        }
    }
}

fn print_help(program: &str) {
    println!("SharPi - AI Coding Assistant");
    println!("");
    println!("USAGE:");
    println!("  {} COMMAND [OPTIONS]", program);
    println!("");
    println!("COMMANDS:");
    println!("  init [--force]         Initialize or reset configuration");
    println!("  chat -m \"message\"      Send a message to the AI");
    println!("  -i                     Enter interactive mode");
    println!("  daemon --start         Start the SharPi daemon");
    println!("  daemon --stop          Stop the SharPi daemon");
    println!("  daemon --status        Check daemon status");
    println!("  --help, -h             Show this help message");
}
