pkg_origin="core"
pkg_name="builder"
pkg_type="composite"
pkg_version="0.1.0"

pkg_services=(
    core/builder-admin
    core/builder-admin-proxy
    core/builder-api
    core/builder-api-proxy
    core/builder-datastore
    core/builder-jobsrv
    core/builder-originsrv
    core/builder-router
    core/builder-sessionsrv
    core/builder-worker
)

pkg_bind_map=(
    [core/builder-api-proxy]="http:core/builder-api"
    [core/builder-api]="router:core/builder-router"
    [core/builder-admin]="router:core/builder-router"
    [core/builder-admin-proxy]="http:core/builder-admin"
    [core/builder-jobsrv]="router:core/builder-router datastore:core/builder-datastore"
    [core/builder-originsrv]="router:core/builder-router datastore:core/builder-datastore"
    [core/builder-sessionsrv]="router:core/builder-router datastore:core/builder-datastore"
    [core/builder-worker]="jobsrv:core/builder-jobsrv depot:core/builder-api"
)
