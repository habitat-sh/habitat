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
  cat > /etc/systemd/system/hab-sup.service <<EOF
[Unit]
Description=Habitat Supervisor
After=network.target

[Service]
Type=simple
ExecStart=/bin/hab sup run
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF
  systemctl daemon-reload
}

@test "Install core packages and prepare for migration" {
  # First install core packages
  run components/hab/install.sh -c stable
  [ "$status" -eq 0 ]
  [ "$(installed_target)" == "x86_64-linux" ]
  
  # Install core/hab-sup
  run hab pkg install core/hab-sup --channel stable
  [ "$status" -eq 0 ]
  [ "$(get_hab_sup_version)" != "Not installed" ]
  
  # Install core/hab-launcher
  run hab pkg install core/hab-launcher --channel stable
  [ "$status" -eq 0 ]
  [ "$(get_hab_launcher_version)" != "Not installed" ]
  
  # Create and start systemd service
  create_systemd_service
  run systemctl start hab-sup.service
  [ "$status" -eq 0 ]
  sleep 5  # Give time for the service to start
  
  run systemctl is-active hab-sup.service
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
  
  run components/hab/migrate.sh
  
  [ "$status" -eq 0 ]
  
  # Check that chef packages are now installed
  [ "$(installed_target chef)" == "x86_64-linux" ]
  [ "$(get_hab_sup_version chef)" != "Not installed" ]
  [ "$(get_hab_launcher_version chef)" != "Not installed" ]
  
  # Verify systemd service is still running
  run systemctl is-active hab-sup.service
  [ "$status" -eq 0 ]
  
  # Check that we're now using chef packages, not core
  new_hab_version="$(installed_version)"
  [[ "$new_hab_version" =~ ^hab\ 2\.[0-9]+\.[0-9]+$ ]]
}
