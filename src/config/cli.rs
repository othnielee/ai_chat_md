use super::constants::*;
use super::types::ChatSource;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help = true,
    before_help = concat!(
        env!("APP_NAME"),
        " v", env!("APP_VERSION"),
        " (build ", env!("APP_BUILD"), ")"
    ),
)]
pub struct CliArgs {
    /// Chat source (-s)
    #[arg(short = 's', long, env = ENV_CHAT_SOURCE)]
    pub chat_source: Option<ChatSource>,

    /// Name for the AI assistant (-a)
    #[arg(short = 'a', long, env = ENV_CHAT_AI_NAME)]
    pub ai_name: Option<String>,

    /// Name for the user (-u)
    #[arg(short = 'u', long, env = ENV_CHAT_USER_NAME)]
    pub user_name: Option<String>,

    /// Title for the chat(-t)
    #[arg(short = 't', long, env = ENV_CHAT_TITLE)]
    pub title: Option<String>,

    /// Timezone for the chat (-z)
    #[arg(short = 'z', long, env = ENV_CHAT_TIMEZONE)]
    pub timezone: Option<String>,

    /// Base directory for input/output files (-d)
    #[arg(short = 'd', long, env = ENV_CHAT_BASE_DIR)]
    pub base_dir: Option<String>,

    /// Force saving of output to same directory as input (-p)
    #[arg(short = 'p', long, env = ENV_CHAT_INLINE_OUTPUT)]
    pub inline_output: Option<bool>,

    /// Input chat file (-i)
    #[arg(short = 'i', long, env = ENV_CHAT_INPUT_FILE)]
    pub input_file: Option<String>,

    /// Output markdown file (-o)
    #[arg(short = 'o', long, env = ENV_CHAT_OUTPUT_FILE)]
    pub output_file: Option<String>,

    /// Path to the config file (-c)
    #[arg(short = 'c', long, env = ENV_APP_CONFIG_PATH)]
    pub config: Option<String>,
}
