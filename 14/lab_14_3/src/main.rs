use std::fs::{File, metadata, set_permissions};
use std::os::unix::fs::PermissionsExt;
use std::io::{stdout, Write};

fn main() -> std::io::Result<()> {
    let file_name = "example.txt";
    let file = File::create(file_name)?;
    let metadata = metadata(file_name)?;
    let permissions = metadata.permissions();
    println!("File created successfully.");
    println!("File permissions: {:?}", permissions);
    let mut new_permissions = permissions.clone();
    new_permissions.set_readonly(true);
    set_permissions(file_name, new_permissions)?;
    let new_metadata = metadata(file_name)?;
    let new_permissions = new_metadata.permissions();
    println!("New file permissions: {:?}", new_permissions);
    Ok(())
}