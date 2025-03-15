use crate::config::MarkdownConfig;
use crate::parser::error::Result;
use crate::parser::model::ClaudeChat;
use crate::parser::participant::ClaudeParticipantMapper;
use crate::parser::timestamp::{TimeFormat, TimeFormatter};
use crate::parser::types::ClaudeContentType;
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::Write;

pub fn parse_to_markdown(chat: &ClaudeChat, config: &MarkdownConfig) -> Result<String> {
    let participant_mapper = ClaudeParticipantMapper::new(config);
    let time_formatter = TimeFormatter::new(&config.timezone, TimeFormat::Rfc3339);
    let mut markdown = String::with_capacity(chat.chat_messages.len() * 500);

    // Setup progress bar
    let progress = ProgressBar::new(chat.chat_messages.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .progress_chars("##-")
            .template("[{bar:40}] {pos}/{len} ({percent}%)")
            .unwrap(),
    );

    // Handle chat title and times
    let title = config.title.clone().unwrap_or_else(|| chat.name.clone());
    let first_message_time = time_formatter.format_iso(&chat.created_at)?;
    let last_message_time = time_formatter.format_iso(&chat.updated_at)?;

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

    // Process the messages
    for message in chat.chat_messages.iter() {
        progress.inc(1);

        // Skip if content is empty
        if message.content.is_empty() {
            continue;
        }

        // Add sender and timestamp as header
        let timestamp = time_formatter.format_iso(&message.created_at)?;
        let sender = participant_mapper.get_name(&message.sender);

        writeln!(markdown, "#### {} @ {}\n", sender, timestamp)?;

        // Process main content
        for content in &message.content {
            match ClaudeContentType::from(content.content_type.as_str()) {
                ClaudeContentType::Text => {
                    if let Some(text) = &content.text {
                        writeln!(markdown, "{}\n", text)?;
                    }
                }
                ClaudeContentType::ToolUse => {
                    if content.name.as_deref() == Some("artifacts") {
                        if let Some(artifact) = &content.artifact {
                            if let Some(id) = &artifact.id {
                                writeln!(markdown, "#### Artifact: {}\n", id)?;
                            }
                            if let Some(content) = &artifact.content {
                                writeln!(markdown, "````")?;
                                writeln!(markdown, "{}", content)?;
                                writeln!(markdown, "````\n")?;
                            }
                            if let Some(code) = &artifact.code {
                                writeln!(markdown, "```")?;
                                writeln!(markdown, "{}", code)?;
                                writeln!(markdown, "```\n")?;
                            }
                        }
                    }
                }
                ClaudeContentType::ToolResult => {
                    // Skip tool result
                    continue;
                }
                ClaudeContentType::Unknown(content_type) => {
                    println!("Encountered unknown content type: {}", content_type);
                    if let Some(text) = &content.text {
                        writeln!(markdown, "{}\n", text)?;
                    }
                }
            }
        }

        // Process attachments
        for attachment in &message.attachments {
            writeln!(markdown, "#### Attachment: {}\n", attachment.file_name)?;
            writeln!(markdown, "````")?;
            writeln!(markdown, "{}", attachment.extracted_content)?;
            writeln!(markdown, "````\n")?;
        }

        writeln!(markdown, "---\n")?;
    }

    progress.finish();

    Ok(markdown)
}
