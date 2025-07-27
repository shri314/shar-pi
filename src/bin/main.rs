use ai_agent::models::claude;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    println!("AI Agent - OpenAI Compatible API Interface");
    
    // Get input text from command line arguments or use default
    let input_text = env::args().nth(1).unwrap_or_else(|| {
        println!("No input provided, using default message");
        "Hello, I'm testing the Claude API integration.".to_string()
    });
    
    println!("Sending request to AI API...");
    
    // Call the Claude API
    match claude::call_claude(&input_text) {
        Ok(response) => {
            println!("\nResponse from AI API:");
            println!("{}", response);
        },
        Err(err) => {
            eprintln!("Error calling AI API: {}", err);
            eprintln!("\nRequired environment variables:");
            eprintln!("- AI_API_KEY: Your API key");
            eprintln!("- AI_API_URL: API endpoint");
            eprintln!("- AI_MODEL: Model to use");
            eprintln!("\nExample:");
            eprintln!("  export AI_API_KEY=your-api-key-here");
            eprintln!("  export AI_API_URL=https://foo.ai/v1/");
            eprintln!("  export AI_MODEL=claude-3-7-sonnet");
        }
    }
    
    Ok(())
}
