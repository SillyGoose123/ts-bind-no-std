use crate::ts::gen_ts_code::FILE_DISCLAIMER;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

pub fn write_to_file(path: &PathBuf, content: &str) -> anyhow::Result<()> {
    let parent = path.parent().ok_or(anyhow::anyhow!(
        "Failed to get parent directory of path: {}",
        path.display()
    ))?;

    create_dir_all(parent)?;

    write(path, content)?;

    Ok(())
}

pub fn write_const_file(path: &PathBuf, imports: Vec<String>, content: &str) -> anyhow::Result<()> {
    let mut builder = String::new();
    builder.write_str(FILE_DISCLAIMER)?;
    builder.write_str("\n\n")?;

    if !path.exists() {
        builder.write_str(imports.join("\n").as_str())?;
        builder.write_str(content)?;
        return write_to_file(path, builder.trim());
    }
    let file = split_file_content(path).expect("Failed to read content from file");

    if file.1.contains(content) {
        return Ok(());
    }

    builder.write_str(&file.0.add(&imports.join("\n")))?;
    builder.write_str(&file.1.add(content))?;

    write_to_file(&path, builder.trim())?;
    Ok(())
}

fn split_file_content(path: &PathBuf) -> anyhow::Result<(String, String)> {
    let reader = BufReader::new(File::open(&path)?);
    let mut content = String::new();
    let mut content_after = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains(FILE_DISCLAIMER) {
            continue;
        }
        if line.contains("import") {
            content.push_str(&line);
            content.push('\n');
            continue
        }

        if line.contains("const") {
            content_after.push_str(&line);
            content_after.push('\n');
        }
    }

    Ok((content, content_after))
}
