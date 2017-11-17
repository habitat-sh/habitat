// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};

use libc;

use {Error, Result};
use coreutils::{chmod, mkdir_p, rmdir, symlink, touch, umask};
use mount::{self, Mount};
use filesystem;
use pty;

const ROOTFS_DIRS: &'static [&'static str] = &[
    "etc",
    "run",
    "src",
    "var",
    "hab/pkgs",
    "hab/cache/keys",
    "hab/cache/artifacts",
];

const WHITELISTED_DEVS: &'static [&'static str] =
    &["null", "random", "full", "tty", "zero", "urandom"];

const READONLY_PATHS: &'static [&'static str] = &[
    "/proc/asound",
    "/proc/bus",
    "/proc/fs",
    "/proc/irq",
    "/proc/sys",
    "/proc/sysrq-trigger",
];

const MASKED_PATHS: &'static [&'static str] = &[
    "/proc/kcore",
    "/proc/latency_stats",
    "/proc/timer_list",
    "/proc/timer_stats",
    "/proc/sched_debug",
    "/sys/firmware",
];

pub fn run(rootfs: &Path, cmd: &OsStr, args: Vec<&OsStr>, mount_artifacts: bool) -> Result<()> {
    let umask_val = 0o0022;
    debug!("setting umask, value={:#o}", umask_val);
    umask(umask_val);

    for entry in ROOTFS_DIRS {
        mkdir_p(rootfs.join(entry))?;
    }

    mount::private("/", Mount::Recursive)?;
    mount::bind(rootfs, rootfs, Mount::Recursive, None)?;

    // Mount `/proc` from namespace
    mkdir_p(rootfs.join("proc"))?;
    mount::procfs(rootfs.join("proc"))?;

    // Create `/dev` filesystem
    let dev = rootfs.join("dev");
    mkdir_p(&dev)?;
    mount::tmpfs(
        "tmpfs",
        &dev,
        Some(libc::MS_NOSUID | libc::MS_NODEV | libc::MS_STRICTATIME),
        Some(0o755),
        Some(65536),
    )?;
    chmod(&dev, 0o0755)?;

    // Create `/dev/pts` filesystem
    let pts = rootfs.join("dev/pts");
    mkdir_p(&pts)?;
    mount::devpts(
        &pts,
        libc::MS_NOSUID | libc::MS_NOEXEC,
        Some(0o0620),
        Some(0o0666),
    )?;
    chmod(&pts, 0o0755)?;

    // Create `/dev/shm` filesystem
    let shm = rootfs.join("dev/shm");
    mkdir_p(&shm)?;
    mount::tmpfs(
        "shm",
        &shm,
        Some(libc::MS_NOSUID | libc::MS_NOEXEC | libc::MS_NODEV),
        Some(0o1777),
        Some(65536),
    )?;

    // Create `/dev/mqueue` filesystem
    let mqueue = rootfs.join("dev/mqueue");
    mkdir_p(&mqueue)?;
    mount::mqueue(&mqueue, libc::MS_NOSUID | libc::MS_NOEXEC | libc::MS_NODEV)?;

    // Create `/sys` filesystem
    let sys = rootfs.join("sys");
    mkdir_p(&sys)?;
    mount::bind(
        "/sys",
        sys,
        Mount::Recursive,
        Some(libc::MS_NOSUID | libc::MS_NOEXEC | libc::MS_NODEV),
    )?;

    // Bind mount whitelisted devices
    let old_umask = umask(0o0000);
    for entry in WHITELISTED_DEVS {
        let source = Path::new("/dev").join(entry);
        let target = rootfs.join("dev").join(entry);
        touch(&target)?;
        mount::bind(&source, &target, Mount::Nonrecursive, None)?;
    }
    umask(old_umask);

    // Setup ptmx symlink
    symlink("pts/ptmx", rootfs.join("dev/ptmx"))?;

    // Setup dev symlinks
    symlink("/proc/self/fd", rootfs.join("dev/fd"))?;
    symlink("/proc/self/fd/0", rootfs.join("dev/stdin"))?;
    symlink("/proc/self/fd/1", rootfs.join("dev/stdout"))?;
    symlink("/proc/self/fd/2", rootfs.join("dev/stderr"))?;
    if Path::new("/proc/kcore").exists() {
        symlink("/proc/kcore", rootfs.join("dev/core"))?;
    }

    // Bind mount /src
    mount::bind(
        env::current_dir()?,
        rootfs.join("src"),
        Mount::Nonrecursive,
        None,
    )?;

    // TODO fn: is this necessary?
    mount::tmpfs("tmpfs", rootfs.join("run"), None, None, None)?;
    symlink("/run", rootfs.join("var/run"))?;

    touch(rootfs.join("etc/hosts"))?;
    {
        let path = rootfs.join("etc/resolv.conf");
        debug!("creating file, path={}", path.display());
        let mut file = File::create(path)?;
        file.write_all("nameserver 8.8.8.8".as_bytes())?;
    }


    // Bind mount Habitat packages and key cache as read-only
    mount::bind(
        Path::new("/hab/pkgs"),
        rootfs.join("hab/pkgs"),
        Mount::Nonrecursive,
        Some(libc::MS_RDONLY),
    )?;
    let source = env::home_dir().ok_or(Error::HomeDirectoryNotFound)?.join(
        ".hab/cache/keys",
    );
    mkdir_p(&source)?;
    mount::bind(
        source,
        rootfs.join("hab/cache/keys"),
        Mount::Nonrecursive,
        Some(libc::MS_RDONLY),
    )?;

    if mount_artifacts {
        // Bind mount outside artifact cache (and ensure outside directory exists)
        let source = env::home_dir().ok_or(Error::HomeDirectoryNotFound)?.join(
            ".hab/cache/artifacts",
        );
        mkdir_p(&source)?;
        mount::bind(
            source,
            rootfs.join("hab/cache/artifacts"),
            Mount::Nonrecursive,
            None,
        )?;
    }

    // Binlink BusyBox and Habitat CLI binaries
    {
        let mut command = Command::new("hab");
        command.args(&["pkg", "binlink", "core/busybox-static"]);
        command.env("FS_ROOT", rootfs);
        command.stdout(Stdio::null());
        debug!("running, command={:?}", &command);
        command.spawn()?.wait()?;
    }
    {
        let mut command = Command::new("hab");
        command.args(&["pkg", "binlink", "core/hab"]);
        command.env("FS_ROOT", rootfs);
        command.stdout(Stdio::null());
        debug!("running, command={:?}", &command);
        command.spawn()?.wait()?;
    }

    // Change the root file system, via `pivot_root(2)`
    mkdir_p(rootfs.join(".pivot_root"))?;
    filesystem::pivot_root(&rootfs, rootfs.join(".pivot_root"))?;
    debug!("setting current directory, path=/");
    env::set_current_dir("/")?;
    mount::private("/.pivot_root", Mount::Recursive)?;
    mount::umount("/.pivot_root", Some(libc::MNT_DETACH))?;
    rmdir("/.pivot_root")?;

    // Set read-only paths
    for entry in READONLY_PATHS {
        let path = Path::new(entry);
        if path.exists() {
            mount::bind(&path, &path, Mount::Recursive, None)?;
            mount::bind(
                &path,
                &path,
                Mount::Recursive,
                Some(libc::MS_RDONLY | libc::MS_REMOUNT),
            )?;
        }
    }

    // Set masked paths
    for entry in MASKED_PATHS {
        let path = Path::new(entry);
        if path.is_dir() {
            mount::tmpfs("tmpfs", &path, Some(libc::MS_RDONLY), None, None)?;
        } else if path.is_file() {
            mount::bind("/dev/null", &path, Mount::Nonrecursive, None)?;
        }
    }

    // Set final umask
    let umask_val = 0o0022;
    debug!("setting umask, value={:#o}", umask_val);
    umask(umask_val);

    // Change into `/src` directory
    let src_path = Path::new("/src");
    debug!("setting current directory, path={}", src_path.display());
    env::set_current_dir(src_path)?;

    // Setup `/dev/console`
    let console = Path::new("/dev/console");
    let master = pty::Master::default()?;
    let ptsname = master.ptsname()?;
    touch(&console)?;
    chmod(&console, 0o0000)?;
    mount::bind(&ptsname, &console, Mount::Nonrecursive, None)?;

    // Finally, call `exec` to become the target program
    exec_command(cmd, args)
}

fn exec_command(cmd: &OsStr, args: Vec<&OsStr>) -> Result<()> {
    let mut command = Command::new(cmd);
    command.args(args);
    command.env("KRANGSCHNAK", "true");
    debug!("calling exec, command={:?}", command);
    command.exec();
    Ok(())
}
