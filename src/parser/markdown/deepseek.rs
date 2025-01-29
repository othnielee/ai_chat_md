use crate::config::MarkdownConfig;
use crate::parser::error::Result;
use crate::parser::model::DeepSeekResponse;
use crate::parser::participant::DeepSeekParticipantMapper;
use crate::parser::timestamp::{TimeFormat, TimeFormatter};
use crate::parser::types::DeepSeekContentType;
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::Write;

pub fn parse_to_markdown(response: &DeepSeekResponse, config: &MarkdownConfig) -> Result<String> {
    let chat = &response.data.biz_data;
    let participant_mapper = DeepSeekParticipantMapper::new(config);
    let time_formatter = TimeFormatter::new(&config.timezone, TimeFormat::Unix);
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
    let title = config
        .title
        .clone()
        .unwrap_or_else(|| chat.chat_session.title.clone());
    let first_message_time = time_formatter.format_unix(chat.chat_session.inserted_at)?;
    let last_message_time = time_formatter.format_unix(chat.chat_session.updated_at)?;

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
    for message in chat.chat_messages.iter() {
        progress.inc(1);

        // Skip if content is empty
        if message.content.is_empty() {
            continue;
        }

        // Add sender and timestamp as header
        let timestamp = time_formatter.format_unix(message.inserted_at)?;
        let sender = participant_mapper.get_name(&message.role);
        writeln!(markdown, "#### {} @ {}\n", sender, timestamp)?;

        // Process content based on type
        match DeepSeekContentType::from("text") {
            DeepSeekContentType::Text => {
                writeln!(markdown, "{}\n", message.content)?;
            }
            DeepSeekContentType::Thinking => {
                if config.reasoning && message.thinking_enabled {
                    if let Some(thinking) = &message.thinking_content {
                        writeln!(markdown, "#### Thinking Process\n")?;
                        writeln!(markdown, "{}\n", thinking)?;
                    }
                }
            }
            DeepSeekContentType::Unknown(content_type) => {
                println!("Encountered unknown content type: {}", content_type);
                writeln!(markdown, "{}\n", message.content)?;
            }
        }

        // Process files if present
        for file in &message.files {
            writeln!(markdown, "#### File: {}\n", file.file_name)?;
            writeln!(
                markdown,
                "Status: {} | Size: {} | Token Usage: {}\n",
                file.status, file.file_size, file.token_usage
            )?;
        }

        writeln!(markdown, "---\n")?;
    }

    progress.finish();

    Ok(markdown)
}
