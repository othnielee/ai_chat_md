use crate::config::MarkdownConfig;
use crate::parser::error::Result;
use crate::parser::model::{ChatGPTChat, ChatGPTContentPart, ChatGPTNode};
use crate::parser::participant::ChatGPTParticipantMapper;
use crate::parser::timestamp::{TimeFormat, TimeFormatter};
use crate::parser::types::ChatGPTContentType;
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::Write;

fn get_ordered_messages(chat: &ChatGPTChat) -> Vec<&ChatGPTNode> {
    let mut messages = Vec::new();
    let mut current_id = Some(chat.current_node.as_str());

    // Walk backwards through the tree to collect messages in reverse order
    while let Some(id) = current_id {
        if let Some(node) = chat.mapping.get(id) {
            if node.message.is_some() {
                // Only collect nodes that have messages
                messages.push(node);
            }
            current_id = node.parent.as_deref();
        } else {
            break;
        }
    }

    // Reverse to get chronological order
    messages.reverse();
    messages
}

pub fn parse_to_markdown(chat: &ChatGPTChat, config: &MarkdownConfig) -> Result<String> {
    let participant_mapper = ChatGPTParticipantMapper::new(config);
    let time_formatter = TimeFormatter::new(&config.timezone, TimeFormat::Unix);

    let messages = get_ordered_messages(chat);
    let mut markdown = String::with_capacity(messages.len() * 500);

    // Setup progress bar
    let progress = ProgressBar::new(messages.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .progress_chars("##-")
            .template("[{bar:40}] {pos}/{len} ({percent}%)")
            .unwrap(),
    );

    // Handle chat title and times
    let title = config.title.clone().unwrap_or_else(|| chat.title.clone());
    let first_message_time = time_formatter.format_unix(chat.created_at)?;
    let last_message_time = time_formatter.format_unix(chat.updated_at)?;

    writeln!(markdown, "# {}", title)?;
    writeln!(markdown, "")?;
    writeln!(
        markdown,
        "**Platform:** {}  ",
        config.chat_source.platform_name()
    )?;
    writeln!(markdown, "**First Message:** {}  ", first_message_time)?;
    writeln!(markdown, "**Last Message:** {}  ", last_message_time)?;
    writeln!(markdown, "\n---\n")?;

    // Process messages
    for node in messages {
        progress.inc(1);

        if let Some(message) = &node.message {
            let content = &message.content;

            // Skip messages authored by tools with reasoning / chain-of-thought metadata
            if message.author.role == "tool" {
                if let Some(initial_text) = &message.metadata.initial_text {
                    if initial_text.trim().eq_ignore_ascii_case("Thinking") {
                        continue;
                    }
                }
            }

            // Skip if there are no text parts or all text parts are empty
            if content.parts.is_empty()
                || content
                    .parts
                    .iter()
                    .filter_map(|part| {
                        if let ChatGPTContentPart::Text(s) = part {
                            Some(s.trim())
                        } else {
                            None
                        }
                    })
                    .all(|s| s.is_empty())
            {
                continue;
            }

            // Add sender and timestamp as header
            let timestamp = if let Some(created_at) = message.created_at {
                time_formatter.format_unix(created_at)?
            } else {
                "Unknown Time".to_string()
            };

            let sender = participant_mapper.get_name(&message.author.role);
            writeln!(markdown, "#### {} @ {}\n", sender, timestamp)?;

            match ChatGPTContentType::from(content.content_type.as_str()) {
                ChatGPTContentType::Text | ChatGPTContentType::MultimodalText => {
                    for part in &content.parts {
                        if let ChatGPTContentPart::Text(text) = part {
                            writeln!(markdown, "{}\n", text)?;
                        }
                    }
                }
                ChatGPTContentType::UserEditableContext => {
                    // Skip user profile/instructions
                    continue;
                }
                ChatGPTContentType::Tool | ChatGPTContentType::System => {
                    for part in &content.parts {
                        if let ChatGPTContentPart::Text(text) = part {
                            writeln!(markdown, "{}\n", text)?;
                        }
                    }
                }
                ChatGPTContentType::Unknown(content_type) => {
                    println!("Encountered unknown content type: {}", content_type);
                    for part in &content.parts {
                        if let ChatGPTContentPart::Text(text) = part {
                            writeln!(markdown, "{}\n", text)?;
                        }
                    }
                }
            }

            writeln!(markdown, "---\n")?;
        }
    }

    progress.finish();

    Ok(markdown)
}
