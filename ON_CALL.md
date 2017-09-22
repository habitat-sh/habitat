# An On Call Core Engineer's guide to Habitat in production or "doing it live"

The Habitat team owns and operates our own services.  We are responsible for their uptime and availability.  To that end, we have a rotating, 72 hour on call period in which each Chef employed team member participates. If you cannot cover your assigned rotation, you are responsible for finding a team mate to fill in for you.
 
## Before you go on call for the first time, make sure you have: 

* Can't figure out how to get access? Ask Jamie Winsor.
* You should be a member of the `core` origin and habitat-sh github group.
* Access to the habitat team [1Password Shared vault](https://team-habitat.1password.com)
* Access to [PagerDuty](https://opscode.pagerduty.com) - ask #helpdesk if you need an account
* Access to [Datadog](https://app.datadoghq.com)
* Access to [Pingdom](https://my.pingdom.com) (creds in Chef LastPass) - Pingdom currenty monitors - https://willem.habitat.sh/v1/depot/origins/core
* Access to [statuspage.io](https://statuspage.io) - Jamie Winsor owns admin
* Ask #helpdesk to add the Habitat AWS Portal app to your OKTA dashboard.
* Ability to ssh to our production environment
* Basic familiarity with the services we run and how they are running
 
## Responsibilities
 
* Available to respond to PagerDuty alerts (if you are going to be away from a computer for an extended period, you are responsible for getting someone to take on call).
* If you are on secondary on-call and receive a page, you are responsible for making sure your help is not needed. 
* You are expected to diagnose, to the best of your ability, and bring in help if needed to resolve.

## Communication Responsibilities
* Declare an operational issue on statuspage.io. This will automatically post an alert to habitat's slack #general channel, update our statuspage, and tweet.
* You are responsible for informing your team that you are taking charge of the issue, and you are responsible for telling your team that you are unable to continue responding to that issue before you stop working on it.
* You are not responsible for responding to questions or community questions. 
* You are responsible for updating statuspage.io as status changes. 
* You are not responsible for posting minutes during the issue if status has not materially changed. All updates go into statuspage.io, no other venue.
* Tone in statuspage.io communication should be friendly and professional.
* You are responsible for escalating immediately to someone more senior or knowledgeable if you cannot fix the issue alone or need help. Never be afraid to start phoning people, regardless of time or day. Getting production into a good state should be your only concern.

## Current state of Production

Each of the builder services runs in its own AWS instance. You can find these instances by logging in to the Habitat AWS portal. Make sure to add the `X-Environment` column to the search results and search for instances in the `live` environment.
 
## Current state of Acceptance:

The acceptance environment should closely mirror the production environment with the exception that it may be running newer service releases. Use the AWS portal as described above to locate the acceptance environment builder service instances.

## Troubleshooting:
 
- Sometimes you need to break the rules. Communicate to your team clearly what rules were broken and how if you do so proper cleanup can occur. You are responsible for cleanup. 
- Never tweak server settings manually. 
- Do not leave different production servers on different Supervisor versions.
- Never deploy a development Supervisor to production. Do a Supervisor release if you need the latest one out. 
- Test or diagnose issues in Acceptance if possible (i.e. if the issue is reproducible).

### Reading logs

You can read the Supervisor output on any of the service instances by running `journalctl -fu hab-sup`. 

### Restarting services

- Most instances just run a single service but there are a couple that run two. 
- Running `systemctl restart hab-sup` will restart the supervisor itself and therefore all services it runs. 
- You can run `sudo hab sup stop [service ident]` and `start` to restart individual services. 
- Run `sudo hab sup status` to determine which services are loaded and what state each service is in.

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
* you can use `psql` but not all the commands will work out of the box.

Each of the builder services occupy their own database. 
- `\l` will list all databases and you can connect to one using `\c <database name>`.
- Because the database is sharded, you need to set your `SEARCH_PATH` to a shard.
- To find which shard you are interested in, you can navigate to a shard using `SET SEARCH_PATH TO shard_<number>;`. `\dn` will list all shards and can be helpful in determining if your database is sharded at all. If it is not, you just need to navigate to `shard_0`. 
- Once set, you can run queries and also use `\dt` to list tables.

If you need to run a query accross all shards, you can run:

```
sudo su - hab
for i in {0..127}; do
     /hab/pkgs/core/postgresql/9.6.1/20170514001355/bin/psql \
        -d builder_originsrv \
        -c "SELECT 'shard_${i}' as shard, name FROM shard_${i}.origins;"; done
```

HINT: If querying the `builder_originsrv` database. All data related to the `core` origin is in `shard_30`.

### Deploying code

If you are in a position where you need to deploy a fix, the builder services (assuming they are up) make this easy. Once your fix is merged to master, you simply need to `Request new build` from the package details page in the depot. You need to be a member of the core origin in order to do this. Once that is done, the supervisor on the builder-origisrv node will update itself.
 
## The Sentinel bot
 
* The sentinel bot lives at bots.habitat.sh
* The SSH key and user for logging in are the same as the production instances
* The sentinel configuration files live at `/hab/svc/sentinel`
* To see what the sentinel service is doing, you can tail the logs with `journalctl -f -u sentinel`
* If you suspect the sentinel bot is stuck and needs to be restarted, you can run `systemctl restart sentinel`
