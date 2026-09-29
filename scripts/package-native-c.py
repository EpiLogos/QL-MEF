#!/usr/bin/env python3
"""Create the native package deterministically on BSD and GNU hosts."""
import gzip
import os
from pathlib import Path
import sys
import tarfile


def package(root: Path, destination: Path) -> None:
    source = root / "ql-mef-c"
    if not source.is_dir():
        raise SystemExit(f"Missing installed native package: {source}")
    temporary = destination.with_suffix(destination.suffix + ".tmp")
    try:
        with temporary.open("wb") as raw:
            with gzip.GzipFile(filename="", mode="wb", fileobj=raw, mtime=0) as compressed:
                with tarfile.open(fileobj=compressed, mode="w", format=tarfile.USTAR_FORMAT) as archive:
                    for path in [source, *sorted(source.rglob("*"))]:
                        info = archive.gettarinfo(str(path), str(path.relative_to(root)))
                        info.uid = info.gid = info.mtime = 0
                        info.uname = info.gname = ""
                        if info.isfile():
                            with path.open("rb") as content:
                                archive.addfile(info, content)
                        else:
                            archive.addfile(info)
        os.replace(temporary, destination)
    finally:
        temporary.unlink(missing_ok=True)


if __name__ == "__main__":
    package(Path(sys.argv[1]), Path(sys.argv[2]))
