#!/bin/bash
set -euo pipefail

while [ ! -f /hab/svc/builder-datastore/config/pwfile ]
do
  sleep 2
done

export PGPASSWORD
PGPASSWORD=$(cat /hab/svc/builder-datastore/config/pwfile)

mkdir -p /hab/svc/builder-api
cat <<EOT > /hab/svc/builder-api/user.toml
log_level = "debug"
[github]
url = "$GITHUB_API_URL"
web_url = "$GITHUB_WEB_URL"
client_id = "$GITHUB_CLIENT_ID"
client_secret = "$GITHUB_CLIENT_SECRET"
app_id = $GITHUB_APP_ID

[web]
app_url          = "http://$APP_HOSTNAME"
community_url    = "https://www.habitat.sh/community"
docs_url         = "https://www.habitat.sh/docs"
environment      = "production"
friends_only     = false
source_code_url  = "https://github.com/habitat-sh/habitat"
tutorials_url    = "https://www.habitat.sh/tutorials"
www_url          = "http://$APP_HOSTNAME/#/sign-in"
EOT

mkdir -p /hab/svc/builder-api-proxy
cat <<EOT > /hab/svc/builder-api-proxy/user.toml
log_level = "debug"

app_url = "http://localhost:9636"

[github]
url = "$GITHUB_API_URL"
web_url = "$GITHUB_WEB_URL"
client_id = "$GITHUB_CLIENT_ID"
client_secret = "$GITHUB_CLIENT_SECRET"
app_id = $GITHUB_APP_ID
app_url = "https://github.com/apps/${GITHUB_APP_NAME}"
EOT

mkdir -p /hab/svc/builder-jobsrv
cat <<EOT > /hab/svc/builder-jobsrv/user.toml

[datastore]
password = "$PGPASSWORD"
database = "builder_jobsrv"

[archive]
backend = "local"
EOT

mkdir -p /hab/svc/builder-originsrv
cat <<EOT > /hab/svc/builder-originsrv/user.toml
log_level = "debug"

[app]
shards = [
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  40,
  41,
  42,
  43,
  44,
  45,
  46,
  47,
  48,
  49,
  50,
  51,
  52,
  53,
  54,
  55,
  56,
  57,
  58,
  59,
  60,
  61,
  62,
  63,
  64,
  65,
  66,
  67,
  68,
  69,
  70,
  71,
  72,
  73,
  74,
  75,
  76,
  77,
  78,
  79,
  80,
  81,
  82,
  83,
  84,
  85,
  86,
  87,
  88,
  89,
  90,
  91,
  92,
  93,
  94,
  95,
  96,
  97,
  98,
  99,
  100,
  101,
  102,
  103,
  104,
  105,
  106,
  107,
  108,
  109,
  110,
  111,
  112,
  113,
  114,
  115,
  116,
  117,
  118,
  119,
  120,
  121,
  122,
  123,
  124,
  125,
  126,
  127
]

[datastore]
password = "$PGPASSWORD"
database = "builder_originsrv"
EOT

mkdir -p /hab/svc/builder-sessionsrv
cat <<EOT > /hab/svc/builder-sessionsrv/user.toml
log_level = "debug"

[app]
shards = [
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  40,
  41,
  42,
  43,
  44,
  45,
  46,
  47,
  48,
  49,
  50,
  51,
  52,
  53,
  54,
  55,
  56,
  57,
  58,
  59,
  60,
  61,
  62,
  63,
  64,
  65,
  66,
  67,
  68,
  69,
  70,
  71,
  72,
  73,
  74,
  75,
  76,
  77,
  78,
  79,
  80,
  81,
  82,
  83,
  84,
  85,
  86,
  87,
  88,
  89,
  90,
  91,
  92,
  93,
  94,
  95,
  96,
  97,
  98,
  99,
  100,
  101,
  102,
  103,
  104,
  105,
  106,
  107,
  108,
  109,
  110,
  111,
  112,
  113,
  114,
  115,
  116,
  117,
  118,
  119,
  120,
  121,
  122,
  123,
  124,
  125,
  126,
  127
]

[datastore]
password = "$PGPASSWORD"
database = "builder_sessionsrv"

[github]
url = "$GITHUB_API_URL"
client_id = "$GITHUB_CLIENT_ID"
client_secret = "$GITHUB_CLIENT_SECRET"
app_id = $GITHUB_APP_ID
EOT

mkdir -p /hab/svc/builder-worker
cat <<EOT > /hab/svc/builder-worker/user.toml
log_level = "debug"

key_dir = "/hab/svc/builder-worker/files"
auto_publish = true
log_level = "debug"
airlock_enabled = false
data_path = "/hab/svc/builder-worker/data"
bldr_url = "http://localhost:9636"
[github]
url = "$GITHUB_API_URL"
web_url = "$GITHUB_WEB_URL"
client_id = "$GITHUB_CLIENT_ID"
client_secret = "$GITHUB_CLIENT_SECRET"
app_id = $GITHUB_APP_ID
EOT