//! Private, atomically replaced native state. An OS advisory lock is released
//! on process death. No recursive scan, public export, raw journal or secrets.
use super::{MAX_BYTES, PersonalRecord, digest};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

fn file_name(reference: &str) -> Result<String, String> {
    let key = reference
        .strip_prefix("ql:nara-record:")
        .ok_or("invalid personal record address")?;
    if key.len() != 64
        || !key
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        return Err("invalid personal record address".into());
    }
    Ok(format!("{key}.json"))
}
#[cfg(unix)]
fn private(path: &Path, directory: bool) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let m = fs::symlink_metadata(path).map_err(|_| "private record is unavailable")?;
    if m.file_type().is_symlink()
        || m.is_dir() != directory
        || (!directory && !m.is_file())
        || m.permissions().mode() & 0o077 != 0
    {
        return Err("private state must be a non-symlink owner-only directory/file".into());
    }
    Ok(())
}
#[cfg(not(unix))]
fn private(_: &Path, _: bool) -> Result<(), String> {
    Err("native personal storage requires an owner-only filesystem adapter on this platform".into())
}
fn ensure(root: &Path) -> Result<(), String> {
    for parent in root.ancestors() {
        if fs::symlink_metadata(parent).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("personal storage path contains a symlink".into());
        }
    }
    if !root.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            let mut builder = fs::DirBuilder::new();
            builder.recursive(true).mode(0o700);
            builder
                .create(root)
                .map_err(|_| "private state directory could not be created")?;
        }
        #[cfg(not(unix))]
        return Err("owner-only personal storage unavailable".into());
    }
    private(root, true)
}
fn open_private(path: &Path, new: bool) -> Result<File, String> {
    let mut options = OpenOptions::new();
    options.read(true).write(new).create_new(new);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }
    options
        .open(path)
        .map_err(|_| "private file could not be opened safely".into())
}
pub(super) fn lock(root: &Path, reference: &str) -> Result<File, String> {
    ensure(root)?;
    let path = root.join(file_name(reference)?.replace(".json", ".lock"));
    let file = match open_private(&path, true) {
        Ok(f) => f,
        Err(_) => {
            private(&path, false)?;
            open_private(&path, false)?
        }
    };
    fs2::FileExt::try_lock_exclusive(&file)
        .map_err(|_| "personal record is busy; no concurrent overwrite was attempted")?;
    Ok(file)
}
pub(super) fn read_optional(
    root: &Path,
    reference: &str,
) -> Result<Option<PersonalRecord>, String> {
    let name = file_name(reference)?;
    if !root.exists() {
        return Ok(None);
    }
    private(root, true)?;
    let path = root.join(name);
    match fs::symlink_metadata(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("personal record metadata unavailable".into()),
        Ok(_) => {}
    }
    private(&path, false)?;
    let mut data = Vec::new();
    open_private(&path, false)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut data)
        .map_err(|_| "private personal record could not be read")?;
    if data.len() as u64 > MAX_BYTES {
        return Err("private personal record exceeds bound".into());
    }
    let record: PersonalRecord = serde_json::from_slice(&data)
        .map_err(|_| "invalid private personal record; bytes retained")?;
    record.validate()?;
    if record.target.record_ref != reference {
        return Err("personal record filename/identity mismatch".into());
    }
    Ok(Some(record))
}
pub(super) fn read(root: &Path, reference: &str) -> Result<PersonalRecord, String> {
    read_optional(root, reference)?
        .ok_or_else(|| "personal record not found; no fixture was substituted".into())
}
pub(super) fn list(root: &Path) -> Result<Vec<serde_json::Value>, String> {
    if !root.exists() {
        return Ok(vec![]);
    }
    private(root, true)?;
    let mut out = Vec::new();
    for entry in fs::read_dir(root).map_err(|_| "private directory unavailable")? {
        let entry = entry.map_err(|_| "private directory entry unavailable")?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".json") {
            continue;
        }
        if out.len() >= 4096 {
            return Err("private record listing exceeds bound".into());
        }
        let r = read(
            root,
            &format!("ql:nara-record:{}", name.trim_end_matches(".json")),
        )?;
        out.push(serde_json::json!({"target":r.target,"revision":r.revision,"event_ref":r.domain.event.event_ref,
            "profile_generation":r.domain.event.profile_generation}));
    }
    out.sort_by_key(|r| r["target"]["record_ref"].as_str().unwrap_or("").to_owned());
    Ok(out)
}
pub(super) fn write(root: &Path, record: &PersonalRecord) -> Result<(), String> {
    private(root, true)?;
    record.validate()?;
    let bytes = serde_json::to_vec(record).map_err(|_| "personal serialization failed")?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("personal state exceeds bound; existing bytes retained".into());
    }
    let name = file_name(&record.target.record_ref)?;
    let destination = root.join(name);
    if destination.exists() {
        private(&destination, false)?;
    }
    let tmp: PathBuf = root.join(format!(
        ".{}-{}.pending",
        std::process::id(),
        digest(&bytes)
    ));
    let mut file = open_private(&tmp, true)?;
    let staged = (|| {
        file.write_all(&bytes)
            .map_err(|_| "private state write failed")?;
        file.sync_all().map_err(|_| "private state sync failed")?;
        fs::rename(&tmp, &destination).map_err(|_| "private state atomic replacement failed")?;
        File::open(root).and_then(|f| f.sync_all()).map_err(|_| {
            "personal commit durability is uncertain; read the same request receipt before retry"
        })?;
        Ok(())
    })();
    if staged.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    staged
}
