use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};

pub(crate) fn rm_contents<T: AsRef<Path>>(path: &T) -> anyhow::Result<()> {
    for entry in fs::read_dir(path)? {
        let fl = entry?;
        if fl.file_type()?.is_dir()
            & (fl.path().to_str().ok_or_else(|| {
                io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Error when converting {} to str", fl.path().display()),
                )
            })? != ".git")
        {
            fs::remove_dir_all(fl.path())?;
        } else {
            fs::remove_file(fl.path())?;
        }
    }

    Ok(())
}

pub(crate) fn copy_all<T: AsRef<Path>>(src: &T, trg: &T) -> anyhow::Result<()> {
    let fls = fs::read_dir(src)?;
    fs::create_dir_all(&trg)?;
    // borrow here so that it can be used later

    for fl in fls {
        let fl = fl?;

        if fl.file_type()?.is_dir() {
            copy_all(&fl.path(), &trg.as_ref().join(fl.file_name()))?;
        } else {
            fs::copy(fl.path(), trg.as_ref().join(fl.file_name()))?;
        }
    }
    Ok(())
}
