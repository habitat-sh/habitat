#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_hab_root
}

teardown() {
    stop_supervisor
}

sup_restarted() {
    local old_pid="${1}"
    local new_pid="$(pgrep 'hab-sup')"
    if [[ "$old_pid" -eq "$new_pid" ]]; then
        return 0
    fi
    return 1
}

@test "supervisor: restart does not chown directories" {
    background ${hab} run
    sleep 2 # give it a sec to come up

    ${hab} pkg install core/runit --binlink

    # start up nginx
    ${hab} pkg install core/nginx
    ${hab} svc load core/nginx
    wait_for_service_to_run nginx

    # create an index.html so there is a page to fetch
    echo "test" > /hab/svc/nginx/data/index.html

    # the nginx children (running as hab) should all have access 
    # to the index.html at this point
    run curl -s -o /dev/null -w "%{http_code}" http://localhost
    [ "$output" = "200" ]

    # remove permissions for the hab user to access the nginx data
    # directory. All index.html requests will now return 403
    chmod g-rwx /hab/svc/nginx/data/
    run curl -s -o /dev/null -w "%{http_code}" http://localhost
    [ "$output" = "403" ]

    local sup_pid="$(pgrep 'hab-sup')"
    kill -s SIGHUP "$sup_pid"

    retry 5 1 sup_restarted "$sup_pid"
    run curl -s -o /dev/null -w "%{http_code}" http://localhost
    [ "$output" = "403" ]
}

