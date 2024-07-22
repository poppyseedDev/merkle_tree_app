use std::fs::File;
use std::io::Write;

pub fn generate_sample_files() -> std::io::Result<()> {
    std::fs::create_dir_all("data")?;
    
    let file1_content = "This is the content of file1.\nIt has multiple lines.\nEach line contains some text.\n";
    let file2_content = "File2 contains different content.\nIt also has multiple lines.\nHere's some more text.\n";
    let file3_content = "The third file, file3, has its own content.\nIt might be similar or different from the others.\nHere are a few more lines of text.\n";

    let mut file1 = File::create("data/file1.txt")?;
    file1.write_all(file1_content.as_bytes())?;

    let mut file2 = File::create("data/file2.txt")?;
    file2.write_all(file2_content.as_bytes())?;

    let mut file3 = File::create("data/file3.txt")?;
    file3.write_all(file3_content.as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    generate_sample_files()
}
