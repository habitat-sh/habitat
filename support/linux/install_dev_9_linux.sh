#!/bin/sh
set -eux

# Install Rust and musl libc target.
# If running as root, put it in a generic location
# and share across all users
# (https://github.com/rust-lang-nursery/rustup.rs/issues/1085#issuecomment-296604244)
if [ "$(whoami)" = "root" ]; then
  export RUSTUP_HOME=/opt/rust
  export CARGO_HOME=/opt/rust
  curl -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path --default-toolchain stable

  cat <<EOF > /usr/local/bin/rustc
  #!/bin/sh

  RUSTUP_HOME=/opt/rust exec /opt/rust/bin/\${0##*/} "\$@"
EOF
  chmod +x  /usr/local/bin/rustc

  cd "${RUSTUP_HOME}/bin" && \
    find ! -name rustc -type f \
      -exec ln -s "/usr/local/bin/rustc" "/usr/local/bin/{}" \;
  rustup target add x86_64-unknown-linux-musl
else # non-root user, install in user directory
  ( # Use a subshell to remove CARGO_HOME since `env -u` is non-POSIX
    unset CARGO_HOME
    curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    . "$HOME/.cargo/env"
    rustup target add x86_64-unknown-linux-musl
  )
fi

rustc --version
cargo --version

if command -v useradd > /dev/null; then
  sudo -E useradd --system --no-create-home hab || true
else
  sudo -E adduser --system hab || true
fi
if command -v groupadd > /dev/null; then
  sudo -E groupadd --system hab || true
else
  sudo -E addgroup --system hab || true
fi

curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh | sudo bash -s -- -c dev-v1.6
sudo HAB_LICENSE="accept-no-persist" hab pkg install core/busybox-static core/hab-studio
