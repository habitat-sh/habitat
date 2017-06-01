# An On Call Engineer's guide to Habitat in production or "doing it live"

The Habitat team owns and operates our own services.  We are responsible for their uptime and availability.  To that end, we have a rotating, 72 hour on call period in which each Chef employed team member participates. If you cannot cover your assigned rotation, you are responsible for getting coverage.
 
## What you need before going on call

* Access to the habitat team [1Password Shared vault](https://team-habitat.1password.com)
* Access to [PagerDuty](https://opscode.pagerduty.com) - ask #helpdesk if you need an account
* Access to [Datadog](https://app.datadoghq.com)
* Access to [Pingdom](https://my.pingdom.com) (creds in Chef LastPass) - Pingdom currenty monitors - https://willem.habitat.sh/v1/depot/origins/core
* Ability to ssh to our production environment (see below)
* Basic familiarity with the services we run and how they are running (detailed below)
 
## Responsibilities
 
* Available to respond to PagerDuty alerts (if you are going to be away from a computer for an extended period, you are responsible for getting someone to take on call).
* Incident response does not mean you need to solve the problem yourself.
* You are expected to diagnose, to the best of your ability, and bring in help if needed.
* Communication while troubleshooting is important.
* Triage incoming GitHub issues and PRs (best effort - at least tag issues if possible and bring critical issues to the forefront).
* Monitor #general and #operations as time permits.

More about Chef’s incident and learning review(post-mortems) can be found at https://chefio.atlassian.net/wiki/pages/viewpage.action?spaceKey=ENG&title=Incidents+and+Post+Mortems 
 
During your on-call rotation, it is expected that you won’t have the same focus on your assigned cards and that you might be more interrupt-driven.  This is a good thing because your teammates can stay focused knowing that their backs are being watched.  You benefit likewise when you are off rotation.
 
## Current state of Production

Each of the builder services runs in its own AWS instance. You can find these instances by logging in to the Habitat AWS portal. If you do not already have access, ask #helpdesk to add the Habitat AWS Portal app to your OKTA dashboard.  Make sure to add the `X-Environment` column to the search results and search for instances in the `live` environment.
 
## Current state of Acceptance:

The acceptance environment should now closely mirror the live environment with the exception that it may be running newer service releases. Use the AWS portal as described above to locate the acceptance environment builder service instances.

## Troubleshooting:
 
Historically, trouble in production can be resolved by restarting services. However, we recently completely revamped all the builder services at the end of May 2017. So for the next few months (I'm writing this on the last day of May..."hi future team!"), we can enjoy a sort of pioneering phase where we learn what new problems our clever new code shall reveal to us. Here are some generic pointers for exploring the status of prduction.

### Reading logs

You can read the supervisor output on any of the service instances by running `journalctl -fu hab-sup`. If you find yourself needing to read production logs, the `-fu` should roll quite naturally off the finger tips.

### Restarting services

Most instances just run a single service but there are a couple that run two. Running `systemctl restart hab-sup` will restart the supervisor itself and therefore all services it runs. You may of course run `sudo hab sup stop [service ident]` and `start` to restart individual services. Run `sudo hab sup status` to determine which services are loaded and what state each service is in.

Here is a brief synopsis of the builder services:

* `api` - acts as the REST API gateway
* `sessionsrv` - manages api authentication and also stores information related to individual accounts. If website authentication is not working or if "My Origins" shows up empty, chances are you need to restart this service.
* `originsrv` - Manages the "depot" data. If package searches is broken, restart this service.
* `router` - this is the hub that routes requests to the appropriate service. If things appear broken and restarting individual services do not resolve site errors, try restarting this service.
* `datastore` - Runs the postgres database. Typically this service does not need restarting unless the logs indicate that it is throwing errors. Be aware that sometimes stopping this service does not clean up well and you may need to clean up some lock files before starting. If that is the case, error messages should state which lock files are causing issues.
* `scheduler`- Manages package builds. If clicking on the `Request new Build` button does nothing, try restarting this service.
* `jobsrv` - Handles build jobs. If you have clicked on the `Request a Build` button and get a popup message that the build was accepted but the build output is never displayed, you may need to restart this service.

### Querying the database

* SSH to the `builder-datastore` instance
* `su` to the `hab` user: `sudo su - hab`
* run `/hab/pkgs/core/postgresql/9.6.1/20170514001355/bin/psql postgres`

Note that while postgres is running as a habitat service and `hab pkg exec` can also run `psql`. `hab pkg exec` will run `psql` with busybox bash loaded and some of the `psql` navigation does not work well.

Each of the builder services occupy their own database. `\l` will list all databases and you can connect to one using `\c <database name>`.

Most of the databases are sharded, so you cannot simply start querying tables until you set your `SEARCH_PATH` to a shard. This can be potentially challenging. You need to know which shard has the data you are interested in. You navigate to a shard using `SET SEARCH_PATH TO shard_<number>;`. `\dn` will list all shards and can be helpful in determining if your database is sharded at all. If it is not, you just need to navigate to `shard_0`. Once set, you can run queries and also use `\dt` to list tables.

In the event that you need to run a query accross all shards, you can run something like this:

```
sudo su - hab
for i in {0..127}; do
     /hab/pkgs/core/postgresql/9.6.1/20170514001355/bin/psql \
        -d builder_originsrv \
        -c "SELECT 'shard_${i}' as shard, name FROM shard_${i}.origins;"; done
```

HINT: If querying the `builder_originsrv` database. All data related to the `core` origin is in `shard_30`.

### Deploying code

If you are in a position where you need to deploy a fix, the builder services (assuming they are up) makes this easy. Once your fix is merged to master, you simply need to `Request new build` from the package details page in the depot. You need to be a member of the core origin in order to do this. Once that is done, the supervisor on the builder-origisrv node will update itself.

Currently there is a bug where a deployment may lead to the need to restart other services. If you are unsure which, its best to simply restart the following:

* sessionsrv
* originsrv
* router
* scheduler
* jobsrv
 
## The Sentinel bot
 
* The sentinel bot lives at bots.habitat.sh
* The SSH key and user for logging in are the same as the production instances
* The sentinel configuration files live at `/hab/svc/sentinel`
* To see what the sentinel service is doing, you can tail the logs with `journalctl -f -u sentinel`
* If you suspect the sentinel bot is stuck and needs to be restarted, you can run `systemctl restart sentinel`
