#[cfg(target_os = "windows")]
fn main() {
    use std::env;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib", manifest_dir);

    let mut res = winres::WindowsResource::new();
    res.set_manifest(
        r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator"/>
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
    );

    match res.compile() {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
