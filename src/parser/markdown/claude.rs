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

    // Setup reasoning handling
    let mut in_reasoning_block = false;

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

        let sender = participant_mapper.get_name(&message.sender);

        // Check if this message contains thinking content
        let has_thinking = message.content.iter().any(|content| {
            matches!(
                ClaudeContentType::from(content.content_type.as_str()),
                ClaudeContentType::Thinking
            )
        });

        // If this message includes thinking and we haven't entered a reasoning block yet, start one
        if config.reasoning && has_thinking && !in_reasoning_block {
            if let Some(thinking_content) = message.content.iter().find(|c| {
                matches!(
                    ClaudeContentType::from(c.content_type.as_str()),
                    ClaudeContentType::Thinking
                )
            }) {
                let timestamp = thinking_content
                    .start_timestamp
                    .as_deref()
                    .unwrap_or(&message.created_at);
                writeln!(
                    markdown,
                    "#### {} @ {}\n",
                    sender,
                    time_formatter.format_iso(timestamp)?
                )?;
                writeln!(markdown, "##### Thinking Process\n")?;
                in_reasoning_block = true;
            }
        }
        // If this message has no thinking content but we were in a reasoning block, end that block
        else if !has_thinking && in_reasoning_block {
            writeln!(markdown, "---\n")?;
            in_reasoning_block = false;
        }

        // If we are not showing reasoning or there is no thinking content, print a normal header
        if !has_thinking || !config.reasoning {
            if let Some(first_text) = message.content.iter().find(|c| {
                matches!(
                    ClaudeContentType::from(c.content_type.as_str()),
                    ClaudeContentType::Text
                )
            }) {
                let timestamp = first_text
                    .start_timestamp
                    .as_deref()
                    .unwrap_or(&message.created_at);
                writeln!(
                    markdown,
                    "#### {} @ {}\n",
                    sender,
                    time_formatter.format_iso(timestamp)?
                )?;
            }
        }

        // Track whether the immediately preceding piece of content was a thinking segment
        let mut last_segment_was_thinking = false;

        // Process main content
        for content in &message.content {
            match ClaudeContentType::from(content.content_type.as_str()) {
                ClaudeContentType::Text => {
                    // If we just had thinking, place a separator and new header so text is separate
                    if last_segment_was_thinking && config.reasoning {
                        writeln!(markdown, "---\n")?;
                        let timestamp = content
                            .start_timestamp
                            .as_deref()
                            .unwrap_or(&message.created_at);
                        writeln!(
                            markdown,
                            "#### {} @ {}\n",
                            sender,
                            time_formatter.format_iso(timestamp)?
                        )?;
                        // We've now transitioned out of the reasoning block
                        in_reasoning_block = false;
                    }
                    if let Some(text) = &content.text {
                        writeln!(markdown, "{}\n", text.trim())?;
                    }
                }
                ClaudeContentType::Thinking => {
                    last_segment_was_thinking = true;
                    // Only render thinking if reasoning mode is enabled
                    if config.reasoning {
                        if let Some(thinking) = &content.thinking {
                            writeln!(markdown, "{}\n", thinking.trim())?;
                        }
                        // Show any provided summaries
                        if let Some(summaries) = &content.summaries {
                            if !summaries.is_empty() {
                                writeln!(markdown, "##### Thinking Summaries\n")?;
                                for (i, summary) in summaries.iter().enumerate() {
                                    writeln!(markdown, "{}. {}\n", i + 1, summary.summary.trim())?;
                                }
                            }
                        }
                    }
                }
                ClaudeContentType::ToolUse => {
                    // If the tool type is 'artifacts', print it as code/artifact content
                    if content.name.as_deref() == Some("artifacts") {
                        if let Some(artifact) = &content.artifact {
                            if let Some(id) = &artifact.id {
                                writeln!(markdown, "#### Artifact: {}\n", id)?;
                            }
                            if let Some(artifact_content) = &artifact.content {
                                writeln!(markdown, "````")?;
                                writeln!(markdown, "{}", artifact_content)?;
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
                        writeln!(markdown, "{}\n", text.trim())?;
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
