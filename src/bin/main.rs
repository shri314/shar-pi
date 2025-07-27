// Copyright (c) 2025 SharPi Contributors
// MIT License

use sharpi::clients::openai;
use sharpi::config;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    println!("SharPi - AI Coding Assistant");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 && args[1] == "config" && args[2] == "init" {
        return config::create_default_config();
    }
    
    let input_text = env::args().nth(1).unwrap_or_else(|| {
        println!("No input provided, using default message");
        "Hello, I'm testing the SharPi AI integration.".to_string()
    });
    
    println!("Sending request to AI API...");
    
    match openai::call_openai(&input_text, None) {
        Ok(response) => {
            println!("\nResponse from AI API:");
            println!("{}", response);
        },
        Err(err) => {
            eprintln!("Error calling AI API: {}", err);
            eprintln!("\nMake sure your configuration is set up correctly:");
            eprintln!("Run 'spi config init' to create a default configuration file");
            eprintln!("Then edit ~/.sharpi/config.toml with your API keys");
        }
    }
    
    Ok(())
}
