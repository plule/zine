use std::{env, fs};

use anyhow::Result;
use time::{format_description, OffsetDateTime};

use crate::ZINE_FILE;

static TEMPLATE_PROJECT_FILE: &str = r#"
[site]
url = "http://localhost"
name = "{name}"
description = ""

[authors]
zine-team = { name ="Zine Team" }
"#;

static TEMPLATE_ISSUE_FILE: &str = r#"
slug = "issue1"
number = 1
title = "Issue 1"

[[article]]
file = "1-first.md"
title = "First article"
author = "zine-team"
cover = ""
pub_date = "{pub_date}"
publish = true
featured = true
"#;

pub fn new_zine_project(name: Option<String>) -> Result<()> {
    let dir = if let Some(name) = name.as_ref() {
        env::current_dir()?.join(name)
    } else {
        env::current_dir()?
    };
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    // Generate project zine.toml
    fs::write(
        dir.join(ZINE_FILE),
        TEMPLATE_PROJECT_FILE.replace("{name}", &name.unwrap_or_default()),
    )?;

    // Create issue dir and issue zine.toml
    let issue_dir = dir.join(crate::ZINE_CONTENT_DIR).join("issue-1");
    fs::create_dir_all(&issue_dir)?;
    let format = format_description::parse("[year]-[month]-[day]")?;
    let today = OffsetDateTime::now_utc().format(&format)?;
    fs::write(
        issue_dir.join(ZINE_FILE),
        TEMPLATE_ISSUE_FILE.replace("{pub_date}", &today),
    )?;

    // Create first article
    fs::write(issue_dir.join("1-first.md"), "Hello Zine")?;
    Ok(())
}
