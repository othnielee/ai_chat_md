use crate::config::MarkdownConfig;
use crate::parser::error::Result;
use crate::parser::model::{ChatGPTChat, ChatGPTNode};
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
    let title = &chat.title;
    let first_message_time = time_formatter.format_unix(chat.created_at)?;
    let last_message_time = time_formatter.format_unix(chat.updated_at)?;

    writeln!(markdown, "# {}", title)?;
    writeln!(markdown, "**Started:** {}  ", first_message_time)?;
    writeln!(markdown, "**Last Message:** {}  ", last_message_time)?;
    writeln!(markdown, "\n---\n")?;

    // Process messages
    for node in messages {
        progress.inc(1);

        if let Some(message) = &node.message {
            let content = &message.content;

            // Skip if content is empty or contains only blank strings
            if content.parts.is_empty() || content.parts.iter().all(|s| s.trim().is_empty()) {
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
                ChatGPTContentType::Text => {
                    for part in &content.parts {
                        writeln!(markdown, "{}\n", part)?;
                    }
                }
                ChatGPTContentType::UserEditableContext => {
                    // Skip user profile/instructions
                    continue;
                }
                ChatGPTContentType::Tool | ChatGPTContentType::System => {
                    for part in &content.parts {
                        writeln!(markdown, "{}\n", part)?;
                    }
                }
                ChatGPTContentType::Unknown(content_type) => {
                    println!("Encountered unknown content type: {}", content_type);
                    for part in &content.parts {
                        writeln!(markdown, "{}\n", part)?;
                    }
                }
            }

            writeln!(markdown, "---\n")?;
        }
    }

    progress.finish();

    Ok(markdown)
}
