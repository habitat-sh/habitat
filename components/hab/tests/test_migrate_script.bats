installed_version() {
  hab --version | cut -d'/' -f1
}

installed_target() {
  local origin="${1:-core}"

  version_release="$(hab --version | cut -d' ' -f2)"
  version="$(cut -d'/' -f1 <<< "$version_release")"
  release="$(cut -d'/' -f2 <<< "$version_release")"
  cat /hab/pkgs/"${origin}"/hab/"$version"/"$release"/TARGET
}

get_hab_sup_version() {
  local origin="${1:-core}"
  if hab pkg list "$origin"/hab-sup &>/dev/null; then
    hab pkg list "$origin"/hab-sup | head -1 | awk '{print $1"/"$2}'
  else
    echo "Not installed"
  fi
}

get_hab_launcher_version() {
  local origin="${1:-core}"
  if hab pkg list "$origin"/hab-launcher &>/dev/null; then
    hab pkg list "$origin"/hab-launcher | head -1 | awk '{print $1"/"$2}'
  else
    echo "Not installed"
  fi
}

create_systemd_service() {
  sudo tee -a /etc/systemd/system/hab-sup.service << 'EOF'
[Unit]
Description=Habitat Supervisor

[Service]
ExecStart=/bin/hab sup run
ExecStop=/bin/hab sup term
KillMode=process

[Install]
WantedBy=default.target
EOF
  sudo systemctl daemon-reload
  sudo systemctl unmask hab-sup
}

# Setup function runs before each test
setup_file() {
  echo "starting setup"
  sudo rm -f /bin/hab
  sudo rm -f /usr/bin/hab
  sudo rm -rf /hab/pkgs/core/hab
  sudo rm -rf /hab/pkgs/chef/hab
  sudo rm -rf /hab/pkgs/core/hab-sup
  sudo rm -rf /hab/pkgs/chef/hab-sup
  sudo rm -rf /hab/pkgs/core/hab-launcher
  sudo rm -rf /hab/pkgs/chef/hab-launcher
}

# Teardown function runs after each test
teardown_file() {
  # Stop any running Habitat services
  if systemctl is-active hab-sup &>/dev/null; then
    echo "Stopping hab-sup service"
    sudo systemctl stop hab-sup
  fi
  
  # Remove systemd service file if it exists
  if [ -f /etc/systemd/system/hab-sup.service ]; then
    echo "Removing hab-sup service file"
    sudo rm -f /etc/systemd/system/hab-sup.service
    sudo systemctl daemon-reload
  fi
  
  echo "Teardown complete"
}

@test "Install core packages and prepare for migration" {
  # First install core packages
  run sudo components/hab/install.sh -c stable
  [ "$status" -eq 0 ]
  [ "$(installed_target)" == "x86_64-linux" ]
  
  # Install core/hab-sup
  run sudo -E hab pkg install core/hab-sup --channel stable
  [ "$status" -eq 0 ]
  [ "$(get_hab_sup_version)" != "Not installed" ]
  
  # Install core/hab-launcher
  run sudo -E hab pkg install core/hab-launcher --channel stable
  [ "$status" -eq 0 ]
  [ "$(get_hab_launcher_version)" != "Not installed" ]
  
  # Create and start systemd service
  sudo hab license accept
  create_systemd_service
  run sudo systemctl start hab-sup
  [ "$status" -eq 0 ]
  sleep 5  # Give time for the service to start
  
  run sudo systemctl is-active hab-sup
  [ "$status" -eq 0 ]
}

@test "Migrate from core to chef packages" {
  # Store initial versions
  initial_hab_version="$(installed_version)"
  initial_hab_sup_version="$(get_hab_sup_version)"
  initial_hab_launcher_version="$(get_hab_launcher_version)"
  
  echo "Initial hab version: $initial_hab_version"
  echo "Initial hab-sup version: $initial_hab_sup_version"
  echo "Initial hab-launcher version: $initial_hab_launcher_version"
  
  run sudo -E components/hab/migrate.sh
  [ "$status" -eq 0 ]
  
  # Check that chef packages are now installed
  [ "$(installed_target chef)" == "x86_64-linux" ]
  [ "$(get_hab_sup_version chef)" != "Not installed" ]
  [ "$(get_hab_launcher_version chef)" != "Not installed" ]
  
  # Verify systemd service is still running
  run systemctl is-active hab-sup
  [ "$status" -eq 0 ]
  
  # Verify that the running hab-sup process is from the chef origin
  run pgrep -a hab-sup
  [ "$status" -eq 0 ]
  [[ "$output" =~ /hab/pkgs/chef/hab-sup ]]
  
  # Check that we're now using chef packages, not core
  new_hab_version="$(installed_version)"
  [[ "$new_hab_version" =~ ^hab\ 2\.[0-9]+\.[0-9]+$ ]]
}

@test "Running migrate.sh again should not restart the hab-sup process" {
  # Get the current PID of hab-sup before running migrate.sh again
  run pgrep -x hab-sup
  [ "$status" -eq 0 ]
  hab_sup_pid_before=$output
  echo "hab-sup PID before second migration: $hab_sup_pid_before"
  
  # Run migrate.sh again
  run sudo -E components/hab/migrate.sh
  [ "$status" -eq 0 ]
  echo "Output from second migration run:"
  echo "$output"
  
  # Check that hab-sup is still running
  run systemctl is-active hab-sup
  [ "$status" -eq 0 ]
  
  # Get the PID of hab-sup after running migrate.sh again
  run pgrep -x hab-sup
  [ "$status" -eq 0 ]
  hab_sup_pid_after=$output
  echo "hab-sup PID after second migration: $hab_sup_pid_after"
  
  # Verify that the PID hasn't changed
  [ "$hab_sup_pid_before" = "$hab_sup_pid_after" ]
}
