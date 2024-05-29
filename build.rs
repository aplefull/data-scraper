use std::io;
use winres;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        res.set_icon("./src/assets/icons/icon.ico");
        res.compile()?;
    }

    Ok(())
}
