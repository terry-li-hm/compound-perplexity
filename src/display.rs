use owo_colors::OwoColorize;
use serde_json::Value;

/// Strip `<think>...</think>` tags from reasoning model output.
fn strip_thinking(text: &str) -> &str {
    if let Some(end) = text.find("</think>") {
        text[end + 8..].trim_start()
    } else {
        text
    }
}

/// Extract citations from the response if present.
fn extract_citations(response: &Value) -> Vec<String> {
    response["citations"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

pub fn display_response(mode: &str, response: &Value) {
    let label = match mode {
        "search" => "search".cyan().to_string(),
        "ask" => "ask".green().to_string(),
        "research" => "research".magenta().to_string(),
        "reason" => "reason".yellow().to_string(),
        _ => mode.to_string(),
    };
    eprintln!("{}", format!("[{label}]").dimmed());

    let content = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No content in response");

    let content = if mode == "reason" {
        strip_thinking(content)
    } else {
        content
    };

    println!("{content}");

    let citations = extract_citations(response);
    if !citations.is_empty() {
        eprintln!("\n{}", "Sources:".dimmed());
        for (i, url) in citations.iter().enumerate() {
            eprintln!("  {}. {}", i + 1, url.dimmed());
        }
    }
}
