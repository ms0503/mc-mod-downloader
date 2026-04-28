use crate::cli::Commands;
use crate::config::Config;
use crate::config::Mod;
use crate::config::Requirement;
use crate::download;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn run(command: Commands, config: Config) -> Result<(), Box<dyn Error>> {
    let Commands::GeneratePage {
        out
    } = command
    else {
        unreachable!()
    };
    let mut body = String::new();
    let mut client_required_mods = String::new();
    let mut client_optional_mods = String::new();
    let mut server_required_mods = String::new();
    let mut server_optional_mods = String::new();
    put_before_body(&mut body);
    put_page_top(&mut body);
    put_section_top(&mut client_required_mods, "Client", "Required");
    put_section_top(&mut client_optional_mods, "Client", "Optional");
    put_section_top(&mut server_required_mods, "Server", "Required");
    put_section_top(&mut server_optional_mods, "Server", "Optional");
    for entry in config.mods {
        if entry.side().client == Requirement::Require {
            put_table_entry(&mut client_required_mods, &entry).await?;
        }
        if entry.side().client == Requirement::Optional {
            put_table_entry(&mut client_optional_mods, &entry).await?;
        }
        if entry.side().server == Requirement::Require {
            put_table_entry(&mut server_required_mods, &entry).await?;
        }
        if entry.side().server == Requirement::Optional {
            put_table_entry(&mut server_optional_mods, &entry).await?;
        }
    }
    put_section_end(&mut client_required_mods);
    put_section_end(&mut client_optional_mods);
    put_section_end(&mut server_required_mods);
    put_section_end(&mut server_optional_mods);
    body.push_str(&client_required_mods);
    body.push_str(&client_optional_mods);
    body.push_str(&server_required_mods);
    body.push_str(&server_optional_mods);
    put_after_body(&mut body);
    let out_dir = match out.parent() {
        None => Path::new("/"),
        Some(dir) =>
            if dir.is_empty() {
                Path::new(".")
            } else {
                dir
            },
    };
    if fs::metadata(out_dir).await.is_err() {
        fs::create_dir_all(out_dir).await?;
    }
    let mut out_file = File::create(out).await?;
    out_file.write_all(body.as_bytes()).await?;
    println!("[INFO] Generated");
    Ok(())
}

fn put_before_body(body: &mut String) {
    body.push_str("<!DOCTYPE html>");
    body.push_str("<html lang=\"en\">");
    body.push_str("<head>");
    body.push_str("<meta charset=\"UTF-8\" />");
    body.push_str("<title>Mod List</title>");
    body.push_str("<style>");
    body.push_str("* {");
    body.push_str("box-sizing: border-box;");
    body.push('}');
    body.push_str("button {");
    body.push_str("display: block flow;");
    body.push('}');
    body.push_str(".table {");
    body.push_str("border: 1px solid #111;");
    body.push_str("border-collapse: collapse;");
    body.push_str("display: block grid;");
    body.push_str("grid-auto-flow: row;");
    body.push_str("width: max-content;");
    body.push('}');
    body.push_str(".table-entry {");
    body.push_str("border-bottom: 1px solid #111;");
    body.push_str("display: block grid;");
    body.push_str("grid-auto-flow: column;");
    body.push_str("grid-column: span 2;");
    body.push_str("grid-template-columns: subgrid;");
    body.push_str("text-align: center;");
    body.push_str("&:last-child {");
    body.push_str("border-bottom: none;");
    body.push('}');
    body.push_str("& > * {");
    body.push_str("border-right: 1px solid #111;");
    body.push_str("padding: 0.5rem 1rem;");
    body.push_str("width: 100%;");
    body.push_str("&:not(.table-header) {");
    body.push_str("place-self: center;");
    body.push('}');
    body.push_str("&:last-child {");
    body.push_str("border-right: none;");
    body.push('}');
    body.push('}');
    body.push('}');
    body.push_str(".table-header {");
    body.push_str("font-weight: bold;");
    body.push('}');
    body.push_str("</style>");
    body.push_str("</head>");
    body.push_str("<body>");
}

fn put_page_top(body: &mut String) {
    body.push_str("<h1>Mod List</h1>");
    body.push_str("<h2>Table of Contents</h2>");
    body.push_str("<ul>");
    body.push_str("<li><a href=\"#client-required\">Client Required Mods</a></li>");
    body.push_str("<li><a href=\"#client-optional\">Client Optional Mods</a></li>");
    body.push_str("<li><a href=\"#server-required\">Server Required Mods</a></li>");
    body.push_str("<li><a href=\"#server-optional\">Server Optional Mods</a></li>");
    body.push_str("</ul>");
}

fn put_section_top(body: &mut String, side: &str, requirement: &str) {
    body.push_str(&format!(
        "<h2 id=\"{}-{}\">{} {} Mods</h2>",
        side.to_lowercase(),
        requirement.to_lowercase(),
        side,
        requirement
    ));
    body.push_str("<div class=\"table\">");
    body.push_str("<div class=\"table-entry\">");
    body.push_str("<div class=\"table-header\">Filename</div>");
    body.push_str("<div class=\"table-header\">Link</div>");
    body.push_str("</div>");
}

async fn put_table_entry(body: &mut String, entry: &Mod) -> Result<(), Box<dyn Error>> {
    body.push_str("<div class=\"table-entry\">");
    body.push_str(&format!("<div>{}</div>", entry.name()));
    body.push_str("<div>");
    match entry {
        Mod::CurseForge {
            name, ..
        } => {
            body.push_str(&format!(
                "<a href=\"https://www.google.com/search?q={}+site%3Awww.curseforge.com\">",
                name
            ));
            body.push_str("<button type=\"button\">Search file</button>");
            body.push_str("</a>");
        }
        Mod::File {
            ..
        } => {
            body.push_str(&format!(
                "<a href=\"{}\">",
                download::file::get_download_url(entry).await?
            ));
            body.push_str("<button type=\"button\">Download file</button>");
            body.push_str("</a>");
        }
        Mod::Modrinth {
            ..
        } => {
            body.push_str(&format!(
                "<a href=\"{}\">",
                download::modrinth::get_download_url(entry).await?
            ));
            body.push_str("<button type=\"button\">Download file</button>");
            body.push_str("</a>");
        }
    }
    body.push_str("</div>");
    body.push_str("</div>");
    Ok(())
}

fn put_section_end(body: &mut String) {
    body.push_str("</div>");
}

fn put_after_body(body: &mut String) {
    body.push_str("</body>");
    body.push_str("</html>");
}
