mod api;
mod models;

use clap::Parser;
use colored::Colorize;
use dialoguer::{Select, theme::ColorfulTheme};
use figlet_rs::FIGfont;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{self, Write};
use std::process;
use tabled::{Table, settings::Style as TableStyle};

use crate::models::RepoRow;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    user: String,
}

fn apply_cyberpunk_gradient(text: &str) -> String {
    let mut result = String::new();
    let start_rgb = (247, 37, 133);
    let end_rgb = (76, 201, 240);

    for line in text.lines() {
        let len = line.chars().count();
        let max_width = std::cmp::max(1, len - 1) as f32;

        for (i, c) in line.chars().enumerate() {
            if c.is_whitespace() {
                result.push(c);
                continue;
            }

            let ratio = i as f32 / max_width;

            let r = (start_rgb.0 as f32 + (end_rgb.0 as f32 - start_rgb.0 as f32) * ratio) as u8;
            let g = (start_rgb.1 as f32 + (end_rgb.1 as f32 - start_rgb.1 as f32) * ratio) as u8;
            let b = (start_rgb.2 as f32 + (end_rgb.2 as f32 - start_rgb.2 as f32) * ratio) as u8;

            result.push_str(&c.to_string().truecolor(r, g, b).to_string());
        }
        result.push('\n');
    }
    result
}

fn get_banner(text: &str) -> Option<String> {
    let font = FIGfont::from_file("asset/ANSI Shadow.flf")
        .unwrap_or_else(|_| FIGfont::standard().unwrap());

    let figure = font.convert(text);
    figure.map(|val| apply_cyberpunk_gradient(&val.to_string()))
}

fn shorten_description(description: Option<String>) -> Option<String> {
    description.map(|desc| {
        let short_desc: String = desc.chars().take(50).collect();
        if desc.len() > 50 {
            format!("{}...", short_desc)
        } else {
            short_desc
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let args = Args::parse();
    let username = &args.user;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner.set_message(format!(
        "Authenticating with GitHub: {}...",
        args.user.cyan()
    ));

    let user = match api::fetch_user(username).await {
        Ok(user) => user,
        Err(e) => {
            spinner.finish_and_clear();
            eprintln!("{} Error: {}", "FAILED".red().bold(), e);
            process::exit(1);
        }
    };

    let mut repository = match api::fetch_repos(username).await {
        Ok(repo) => repo,
        Err(e) => {
            spinner.finish_and_clear();
            eprintln!("{} Error: {}", "FAILED".red().bold(), e);
            process::exit(1);
        }
    };

    spinner.finish_and_clear();

    let mut top_langs = HashMap::<String, i32>::new();
    repository.iter().for_each(|f| {
        let binding = "Unknown".to_string();
        let lang = f.language.as_ref().unwrap_or(&binding);
        *top_langs.entry(lang.clone()).or_insert(0) += 1;
    });

    let mut top_langs_vec: Vec<(&String, &i32)> = top_langs.iter().collect();
    top_langs_vec.sort_by(|a, b| b.1.cmp(a.1));
    let top_langs_sorted = top_langs_vec.into_iter().take(3).collect::<Vec<_>>();

    repository.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    let table_rows: Vec<RepoRow> = repository
        .iter()
        .take(5)
        .map(|r| RepoRow {
            name: r.name.clone().cyan().bold().to_string(),
            stars: r.stargazers_count,
            language: r
                .language
                .clone()
                .unwrap_or("-".to_string())
                .magenta()
                .to_string(),
            description: shorten_description(r.description.clone())
                .unwrap_or_default()
                .dimmed()
                .to_string(),
        })
        .collect();

    let table = Table::new(table_rows)
        .with(TableStyle::rounded())
        .to_string();

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    if let Some(banner) = get_banner("GIT SPY") {
        writeln!(writer, "{}", banner)?;
    }

    let separator = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".truecolor(60, 60, 60);

    writeln!(writer, "{}", separator)?;

    let profile_url = format!("https://github.com/{}", user.login);
    let at_username = format!("{}{}", "@".cyan(), user.login.magenta().bold());
    let link = format!(
        "\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\",
        url = profile_url,
        text = at_username
    );

    writeln!(writer, " TARGET IDENTIFIED: {}", link)?;
    writeln!(writer, "{}", separator)?;

    let label_width = 12;

    if let Some(name) = user.name {
        writeln!(
            writer,
            " {:<w$} {}",
            "Name:",
            name.white().bold(),
            w = label_width
        )?;
    }

    let bio = user.bio.unwrap_or_default();
    if !bio.is_empty() {
        writeln!(
            writer,
            " {:<w$} {}",
            "Bio:",
            bio.italic().dimmed(),
            w = label_width
        )?;
    }

    if let Some(loc) = user.location {
        writeln!(
            writer,
            " {:<w$} {}",
            "Location:",
            loc.cyan(),
            w = label_width
        )?;
    }

    writeln!(
        writer,
        " {:<w$} {}",
        "Followers:",
        user.followers.to_string().green(),
        w = label_width
    )?;
    writeln!(
        writer,
        " {:<w$} {}",
        "Repos:",
        user.public_repos.to_string().yellow(),
        w = label_width
    )?;

    if !top_langs_sorted.is_empty() {
        writeln!(writer, "{}", separator)?;

        let formatted_langs: Vec<String> = top_langs_sorted
            .iter()
            .map(|(lang, count)| {
                format!("{} {}", lang.blue().bold(), format!("({})", count).dimmed())
            })
            .collect();

        writeln!(
            writer,
            " STACK: {}",
            formatted_langs.join(&"  |  ".dimmed().to_string())
        )?;
    }

    writeln!(writer, "{}", separator)?;
    writeln!(writer, " TOP ARTIFACTS")?;
    writeln!(writer, "{}", separator)?;
    writeln!(writer, "{}", table)?;
    writeln!(writer)?;

    writer.flush()?;

    let repo_names: Vec<&String> = repository.iter().take(5).map(|r| &r.name).collect();

    println!();

    let theme = ColorfulTheme {
        prompt_style: console::Style::new().cyan().bold(),
        active_item_style: console::Style::new().magenta().bold(),
        unchecked_item_prefix: console::style("○".to_string()),
        checked_item_prefix: console::style("◉".to_string()).green(),
        ..ColorfulTheme::default()
    };

    let selection = Select::with_theme(&theme)
        .with_prompt("Select uplink channel (ESC to cancel)")
        .default(0)
        .items(&repo_names)
        .interact_opt()?;

    if let Some(index) = selection {
        let url = &repository[index].html_url;
        println!("Establishing secure connection to {}...", url.cyan());
        if let Err(_) = webbrowser::open(url) {
            eprintln!("Connection failed.");
        }
    } else {
        println!("{}", "Session terminated.".dimmed());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_description_under_limit() {
        let text = "Kurzer Text".to_string();
        let result = shorten_description(Some(text.clone()));
        assert_eq!(result, Some(text));
    }

    #[test]
    fn test_shorten_description_over_limit() {
        let text = "Dies ist ein sehr langer Text, der definitiv über fünfzig Zeichen lang ist und gekürzt werden muss.".to_string();
        let result = shorten_description(Some(text));
        let result_str = result.unwrap();

        assert!(result_str.ends_with("..."));
        assert!(result_str.chars().count() <= 53);
    }

    #[test]
    fn test_shorten_description_none() {
        let result = shorten_description(None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_cyberpunk_gradient_not_empty() {
        let input = "Cyberpunk";
        let output = apply_cyberpunk_gradient(input);

        assert!(output.len() > input.len());
        assert!(output.contains('\n'));
    }

    #[test]
    fn test_get_banner_returns_something() {
        let banner = get_banner("TEST");
        assert!(banner.is_some());

        let banner_str = banner.unwrap();
        assert!(!banner_str.trim().is_empty());
    }

    #[test]
    fn test_repo_row_creation() {
        let row = RepoRow {
            name: "TestRepo".to_string(),
            stars: 42,
            language: "Rust".to_string(),
            description: "Desc".to_string(),
        };

        assert_eq!(row.name, "TestRepo");
        assert_eq!(row.stars, 42);
    }
}
