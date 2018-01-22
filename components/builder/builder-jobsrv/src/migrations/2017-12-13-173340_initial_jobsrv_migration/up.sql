CREATE SEQUENCE IF NOT EXISTS job_id_seq;

CREATE TABLE IF NOT EXISTS jobs (
    id bigint PRIMARY KEY DEFAULT next_id_v1('job_id_seq'),
    owner_id bigint,
    job_state text,
    project_id bigint,
    project_name text,
    project_owner_id bigint,
    project_plan_path text,
    vcs text,
    vcs_arguments text[],
    net_error_code int,
    net_error_msg text,
    scheduler_sync bool DEFAULT false,
    created_at timestamptz DEFAULT now(),
    updated_at timestamptz
);

CREATE OR REPLACE FUNCTION insert_job_v1 (
  owner_id bigint,
  project_id bigint,
  project_name text,
  project_owner_id bigint,
  project_plan_path text,
  vcs text,
  vcs_arguments text[]
  ) RETURNS SETOF jobs AS $$
      BEGIN
          RETURN QUERY INSERT INTO jobs (owner_id, job_state, project_id, project_name, project_owner_id, project_plan_path, vcs, vcs_arguments)
              VALUES (owner_id, 'Pending', project_id, project_name, project_owner_id, project_plan_path, vcs, vcs_arguments)
              RETURNING *;
          RETURN;
      END
$$ LANGUAGE plpgsql VOLATILE;

-- Hey, Adam - why did you do `select *` here? Isn't that bad?
--
-- So glad you asked. In this case, it's better - essentially we have an API call that
-- returns a job object, which is flattened into the table structure above. We then
-- translate those job rows into Job structs. Since the table design is purely additive,
-- this allows us to add data to the table without having to re-roll functions that
-- generate Job structs, and keeps things DRY.
--
-- Just make sure you always address the columns by name, not by position.

CREATE OR REPLACE FUNCTION get_job_v1 (jid bigint) RETURNS SETOF jobs AS $$
BEGIN
  RETURN QUERY SELECT * FROM jobs WHERE id = jid;
  RETURN;
END
$$ LANGUAGE plpgsql STABLE;


-- The pending_jobs function acts as an internal queue. It looks for jobs that are
-- 'Pending', and sorts them according to the time they were entered - first in, first out.
--
-- You can pass this function a number of jobs to return - it will return up-to that many.
--
-- The way it works internally - we select the rows for update, skipping rows that are
-- already locked. That means multiple jobsrvs, or multiple worker managers, can run in
-- parallel against the table - they will simply skip other rows that are currently on the
-- way out the door.
--
-- Any row selected gets its state updated to "Dispatched" before being sent back, ensuring
-- that no other worker could receive the job. If we fail to dispatch the job, its the
-- callers job to change the jobs status back to "Pending", which puts it back in the
-- queue.
--
-- Note that the sort order ensures that jobs that fail to dispatch and are then returned
-- will be the first job selected, making FIFO a reality.

CREATE OR REPLACE FUNCTION pending_jobs_v1 (integer) RETURNS SETOF jobs AS
$$
DECLARE
  r jobs % rowtype;
BEGIN
  FOR r IN
    SELECT * FROM jobs
    WHERE job_state = 'Pending'
    ORDER BY created_at ASC
    FOR UPDATE SKIP LOCKED
    LIMIT $1
  LOOP
    UPDATE jobs SET job_state='Dispatched', scheduler_sync=false, updated_at=now() WHERE id=r.id RETURNING * INTO r;
    RETURN NEXT r;
  END LOOP;
  RETURN;
END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION sync_jobs_v1() RETURNS SETOF jobs AS $$
BEGIN
    RETURN QUERY SELECT * FROM jobs WHERE scheduler_sync = false;
    RETURN;
END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION set_jobs_sync_v1(in_job_id bigint) RETURNS VOID AS $$
BEGIN
  UPDATE jobs SET scheduler_sync = true WHERE id = in_job_id;
END
$$ LANGUAGE plpgsql VOLATILE;

DROP INDEX IF EXISTS pending_jobs_index_v1;

CREATE INDEX pending_jobs_index_v1 on jobs(created_at) WHERE job_state = 'Pending';


-- We're deliberately returning only the 50 most
-- recently-created jobs here. A future version of this
-- function may take additional parameters for sorting,
-- filtering, and pagination.
--
-- Also deliberately using `SELECT *` here, for the same
-- reasons listed above for `get_job_v1`.
--
-- Note that `project_name` here is an origin-qualified
-- project name, e.g. "core/nginx".


CREATE OR REPLACE FUNCTION get_jobs_for_project_v1(p_project_name TEXT)
RETURNS SETOF jobs
LANGUAGE SQL STABLE AS $$
  SELECT *
  FROM jobs
  WHERE project_name = p_project_name
  ORDER BY created_at DESC
  LIMIT 50;
$$;

ALTER TABLE jobs ADD COLUMN IF NOT EXISTS build_started_at TIMESTAMPTZ DEFAULT NULL;

ALTER TABLE jobs ADD COLUMN IF NOT EXISTS build_finished_at TIMESTAMPTZ DEFAULT NULL;

ALTER TABLE jobs ADD COLUMN IF NOT EXISTS package_ident TEXT DEFAULT NULL;

CREATE OR REPLACE FUNCTION update_job_v1(
  p_job_id bigint,
  p_state text,
  p_build_started_at timestamptz,
  p_build_finished_at timestamptz,
  p_package_ident text)
RETURNS VOID
LANGUAGE SQL VOLATILE AS $$
  UPDATE jobs
  SET job_state = p_state,
      scheduler_sync = false,
      updated_at = now(),
      build_started_at = p_build_started_at,
      build_finished_at = p_build_finished_at,
      package_ident = p_package_ident
  WHERE id = p_job_id;
$$;

ALTER TABLE jobs ADD COLUMN IF NOT EXISTS archived BOOLEAN DEFAULT FALSE NOT NULL;

CREATE OR REPLACE FUNCTION mark_as_archived_v1(p_job_id BIGINT)
RETURNS VOID
LANGUAGE SQL VOLATILE as $$
  UPDATE jobs
  SET archived = TRUE
  WHERE id = p_job_id;
$$;

CREATE OR REPLACE FUNCTION update_job_v2(
  p_job_id bigint,
  p_state text,
  p_build_started_at timestamptz,
  p_build_finished_at timestamptz,
  p_package_ident text,
  p_err_code int,
  p_err_msg text)
RETURNS VOID
LANGUAGE SQL VOLATILE AS $$
  UPDATE jobs
  SET job_state = p_state,
      scheduler_sync = false,
      updated_at = now(),
      build_started_at = p_build_started_at,
      build_finished_at = p_build_finished_at,
      package_ident = p_package_ident,
      net_error_code = p_err_code,
      net_error_msg = p_err_msg
  WHERE id = p_job_id;
$$;

CREATE OR REPLACE FUNCTION get_jobs_for_project_v2(p_project_name TEXT, p_limit bigint, p_offset bigint)
RETURNS TABLE (total_count bigint, id bigint, owner_id bigint, job_state text, created_at timestamptz,
              build_started_at timestamptz, build_finished_at timestamptz, package_ident text,
              project_id bigint, project_name text, project_owner_id bigint, project_plan_path text,
              vcs text, vcs_arguments text[], net_error_msg text, net_error_code integer, archived boolean)
LANGUAGE SQL STABLE AS $$
  SELECT COUNT(*) OVER () AS total_count, id, owner_id, job_state, created_at, build_started_at,
  build_finished_at, package_ident, project_id, project_name, project_owner_id, project_plan_path, vcs,
  vcs_arguments, net_error_msg, net_error_code, archived
  FROM jobs
  WHERE project_name = p_project_name
  ORDER BY created_at DESC
  LIMIT p_limit
  OFFSET p_offset;
$$;

ALTER TABLE jobs ADD COLUMN IF NOT EXISTS channel TEXT DEFAULT NULL;

CREATE OR REPLACE FUNCTION insert_job_v2 (
  p_owner_id bigint,
  p_project_id bigint,
  p_project_name text,
  p_project_owner_id bigint,
  p_project_plan_path text,
  p_vcs text,
  p_vcs_arguments text[],
  p_channel text
  ) RETURNS SETOF jobs AS $$
      INSERT INTO jobs (owner_id, job_state, project_id, project_name, project_owner_id, project_plan_path, vcs, vcs_arguments, channel)
      VALUES (p_owner_id, 'Pending', p_project_id, p_project_name, p_project_owner_id, p_project_plan_path, p_vcs, p_vcs_arguments, p_channel)
      RETURNING *;
$$ LANGUAGE SQL VOLATILE;


CREATE TABLE IF NOT EXISTS busy_workers (
  ident text,
  job_id bigint,
  quarantined bool,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  UNIQUE(ident, job_id)
);

CREATE OR REPLACE FUNCTION upsert_busy_worker_v1 (
    in_ident text,
    in_job_id bigint,
    in_quarantined bool
) RETURNS SETOF busy_workers AS $$
  BEGIN
    RETURN QUERY INSERT INTO busy_workers (ident, job_id, quarantined)
    VALUES (in_ident, in_job_id, in_quarantined)
    ON CONFLICT(ident, job_id)
    DO UPDATE SET quarantined=in_quarantined RETURNING *;
    RETURN;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_busy_workers_v1()
RETURNS SETOF busy_workers AS $$
  SELECT * FROM busy_workers
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION delete_busy_worker_v1 (
  in_ident text,
  in_job_id bigint
) RETURNS void AS $$
  DELETE FROM busy_workers
  WHERE ident = in_ident AND job_id = in_job_id
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION get_cancel_pending_jobs_v1()
RETURNS SETOF jobs
LANGUAGE SQL VOLATILE AS $$
  SELECT *
  FROM jobs
  WHERE job_state = 'CancelPending'
$$;

DROP FUNCTION IF EXISTS reset_jobs_v1();

ALTER TABLE jobs ADD COLUMN IF NOT EXISTS sync_count INTEGER DEFAULT 0;

CREATE OR REPLACE FUNCTION sync_jobs_v2() RETURNS SETOF jobs AS $$
  SELECT * FROM jobs WHERE (scheduler_sync = false) OR (sync_count > 0);
$$ LANGUAGE SQL STABLE;

CREATE OR REPLACE FUNCTION set_jobs_sync_v2(in_job_id bigint) RETURNS VOID AS $$
  UPDATE jobs SET scheduler_sync = true, sync_count = sync_count-1 WHERE id = in_job_id;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION update_job_v3(
  p_job_id bigint,
  p_state text,
  p_build_started_at timestamptz,
  p_build_finished_at timestamptz,
  p_package_ident text,
  p_err_code int,
  p_err_msg text)
RETURNS VOID
LANGUAGE SQL VOLATILE AS $$
  UPDATE jobs
  SET job_state = p_state,
      scheduler_sync = false,
      sync_count = sync_count + 1,
      updated_at = now(),
      build_started_at = p_build_started_at,
      build_finished_at = p_build_finished_at,
      package_ident = p_package_ident,
      net_error_code = p_err_code,
      net_error_msg = p_err_msg
  WHERE id = p_job_id;
$$;

ALTER TABLE jobs ADD COLUMN IF NOT EXISTS worker TEXT DEFAULT NULL;

CREATE OR REPLACE FUNCTION next_pending_job_v1 (p_worker text) RETURNS SETOF jobs AS
$$
DECLARE
    r jobs % rowtype;
BEGIN
    FOR r IN
        SELECT * FROM jobs
        WHERE job_state = 'Pending'
        ORDER BY created_at ASC
        FOR UPDATE SKIP LOCKED
        LIMIT 1
    LOOP
        UPDATE jobs SET job_state='Dispatched', scheduler_sync=false, worker=p_worker, updated_at=now()
        WHERE id=r.id
        RETURNING * INTO r;
        RETURN NEXT r;
    END LOOP;
  RETURN;
END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_dispatched_jobs_v1()
RETURNS SETOF jobs
LANGUAGE SQL STABLE AS $$
  SELECT *
  FROM jobs
  WHERE job_state = 'Dispatched'
$$;

CREATE TABLE IF NOT EXISTS graph_packages (
  id bigserial PRIMARY KEY,
  ident text UNIQUE,
  deps text[],
  target TEXT DEFAULT NULL,
  created_at timestamptz DEFAULT now()
);

CREATE SEQUENCE IF NOT EXISTS groups_id_seq;

CREATE TABLE IF NOT EXISTS groups (
  id bigint PRIMARY KEY DEFAULT next_id_v1('groups_id_seq'),
  group_state text,
  project_name text DEFAULT NULL,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

DROP INDEX IF EXISTS pending_groups_index_v1;

CREATE INDEX pending_groups_index_v1 on groups(created_at) WHERE group_state = 'Pending';

CREATE TABLE IF NOT EXISTS group_projects (
  id bigserial PRIMARY KEY,
  owner_id bigint,
  project_name text,
  project_ident text,
  project_state text,
  job_id bigint DEFAULT 0,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE OR REPLACE FUNCTION upsert_graph_package_v1 (
  in_ident text,
  in_deps text[],
  in_target text
) RETURNS SETOF graph_packages AS $$
  BEGIN
    RETURN QUERY INSERT INTO graph_packages (ident, deps, target)
    VALUES (in_ident, in_deps, in_target)
    ON CONFLICT(ident)
    DO UPDATE SET deps=in_deps, target=in_target RETURNING *;
    RETURN;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION get_graph_packages_v1 () RETURNS SETOF graph_packages AS $$
BEGIN
  RETURN QUERY SELECT * FROM graph_packages;
  RETURN;
END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION count_graph_packages_v1 (origin text) RETURNS bigint AS $$
BEGIN
  RETURN COUNT(*) FROM graph_packages WHERE ident ~ ('^' || origin || '/');
END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_graph_package_v1 (pident text) RETURNS SETOF graph_packages AS $$
BEGIN
  RETURN QUERY SELECT * FROM graph_packages
  WHERE ident = pident;
  RETURN;
END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION insert_group_v1 (
  root_project text,
  project_names text[],
  project_idents text[]
) RETURNS SETOF groups
  LANGUAGE SQL
  VOLATILE AS $$
  WITH my_group AS (
          INSERT INTO groups (project_name, group_state)
          VALUES (root_project, 'Pending') RETURNING *
      ), my_project AS (
          INSERT INTO group_projects (owner_id, project_name, project_ident, project_state)
          SELECT g.id, project_info.name, project_info.ident, 'NotStarted'
          FROM my_group AS g, unnest(project_names, project_idents) AS project_info(name, ident)
      )
  SELECT * FROM my_group;
$$;

CREATE OR REPLACE FUNCTION get_group_v1 (gid bigint) RETURNS SETOF groups AS $$
BEGIN
  RETURN QUERY SELECT * FROM groups WHERE id = gid;
  RETURN;
END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION get_group_projects_for_group_v1 (gid bigint) RETURNS SETOF group_projects AS $$
  BEGIN
    RETURN QUERY SELECT * FROM group_projects WHERE owner_id = gid;
    RETURN;
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION count_group_projects_v1 (origin text) RETURNS bigint AS $$
  BEGIN
    RETURN COUNT(*) FROM group_projects WHERE project_ident ~ ('^' || origin || '/');
  END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION pending_groups_v1 (integer) RETURNS SETOF groups AS $$
DECLARE
    r groups % rowtype;
BEGIN
    FOR r IN
        SELECT * FROM groups
        WHERE group_state = 'Pending'
        ORDER BY created_at ASC
        FOR UPDATE SKIP LOCKED
        LIMIT $1
    LOOP
        UPDATE groups SET group_state='Dispatching', updated_at=now() WHERE id=r.id RETURNING * INTO r;
        RETURN NEXT r;
    END LOOP;
  RETURN;
END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION set_group_state_v1 (gid bigint, gstate text) RETURNS void AS $$
  BEGIN
      UPDATE groups SET group_state=gstate, updated_at=now() WHERE id=gid;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION set_group_project_state_v1 (pid bigint, jid bigint, state text) RETURNS void AS $$
  BEGIN
    UPDATE group_projects SET project_state=state, job_id=jid, updated_at=now() WHERE id=pid;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION find_group_project_v1 (gid bigint, name text) RETURNS SETOF group_projects AS $$
BEGIN
  RETURN QUERY SELECT * FROM group_projects WHERE owner_id = gid AND project_name = name;
  RETURN;
END
$$ LANGUAGE plpgsql STABLE;

CREATE OR REPLACE FUNCTION set_group_project_name_state_v1 (gid bigint, pname text, state text) RETURNS void AS $$
  BEGIN
    UPDATE group_projects SET project_state=state, updated_at=now() WHERE owner_id=gid AND project_name=pname;
  END
$$ LANGUAGE plpgsql VOLATILE;

CREATE OR REPLACE FUNCTION set_group_project_state_ident_v1 (pid bigint, jid bigint, state text, ident text) RETURNS void AS $$
  UPDATE group_projects SET project_state=state, job_id=jid, project_ident=ident, updated_at=now() WHERE id=pid;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION check_active_group_v1(pname text) RETURNS SETOF groups AS $$
  SELECT * FROM groups
  WHERE project_name = pname
  AND group_state IN ('Pending', 'Dispatching')
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION count_unique_graph_packages_v1 (
  op_origin text
) RETURNS bigint
LANGUAGE SQL STABLE AS $$
  WITH idents AS (
    SELECT regexp_split_to_array(ident, '/') as parts
    FROM graph_packages
  )
  SELECT COUNT(DISTINCT i.parts[2]) AS total
  FROM idents i
  WHERE i.parts[1] = op_origin
$$;

CREATE OR REPLACE FUNCTION abort_group_v1(in_gid bigint) RETURNS void AS $$
  UPDATE group_projects SET project_state='Failure'
    WHERE owner_id = in_gid
    AND (project_state = 'InProgress' OR project_state = 'NotStarted');
  UPDATE groups SET group_state='Complete' where id = in_gid;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION insert_group_v2 (
  root_project text,
  project_names text[],
  project_idents text[]
) RETURNS SETOF groups
  LANGUAGE SQL
  VOLATILE AS $$
  WITH my_group AS (
          INSERT INTO groups (project_name, group_state)
          VALUES (root_project, 'Queued') RETURNING *
      ), my_project AS (
          INSERT INTO group_projects (owner_id, project_name, project_ident, project_state)
          SELECT g.id, project_info.name, project_info.ident, 'NotStarted'
          FROM my_group AS g, unnest(project_names, project_idents) AS project_info(name, ident)
      )
  SELECT * FROM my_group;
$$;

CREATE OR REPLACE FUNCTION get_queued_group_v1 (pname text) RETURNS SETOF groups AS $$
  SELECT * FROM groups
  WHERE project_name = pname
  AND group_state = 'Queued'
$$ LANGUAGE SQL VOLATILE;

DROP INDEX IF EXISTS queued_groups_index_v1;

CREATE INDEX queued_groups_index_v1 on groups(created_at) WHERE group_state = 'Queued';

CREATE OR REPLACE FUNCTION get_queued_groups_v1 () RETURNS SETOF groups AS $$
  SELECT * FROM groups
  WHERE group_state = 'Queued'
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION cancel_group_v1(in_gid bigint) RETURNS void AS $$
  UPDATE group_projects SET project_state='Canceled'
    WHERE owner_id = in_gid
    AND (project_state = 'NotStarted');
  UPDATE groups SET group_state='Canceled' where id = in_gid;
$$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION get_job_groups_for_origin_v1 (
  op_origin text
) RETURNS SETOF groups AS $$
  SELECT *
  FROM groups
  WHERE project_name LIKE (op_origin || '/%')
  ORDER BY project_name
$$ LANGUAGE SQL STABLE;
