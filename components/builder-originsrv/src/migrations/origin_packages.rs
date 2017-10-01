// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use db::migration::Migrator;

use error::SrvResult;

pub fn migrate(migrator: &mut Migrator) -> SrvResult<()> {
    migrator.migrate(
        "originsrv",
        r#"CREATE SEQUENCE IF NOT EXISTS origin_package_id_seq;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_packages (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_package_id_seq'),
                    origin_id bigint REFERENCES origins(id),
                    owner_id bigint,
                    name text,
                    ident text UNIQUE,
                    checksum text,
                    manifest text,
                    config text,
                    target text,
                    deps text,
                    tdeps text,
                    exposes text,
                    scheduler_sync bool DEFAULT false,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#,
    )?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_origin_package_v1 (
                    op_origin_id bigint,
                    op_owner_id bigint,
                    op_name text,
                    op_ident text,
                    op_checksum text,
                    op_manifest text,
                    op_config text,
                    op_target text,
                    op_deps text,
                    op_tdeps text,
                    op_exposes text
                 ) RETURNS SETOF origin_packages AS $$
                     DECLARE
                        inserted_package origin_packages;
                        channel_id bigint;
                     BEGIN
                         INSERT INTO origin_packages (origin_id, owner_id, name, ident, checksum, manifest, config, target, deps, tdeps, exposes)
                                VALUES (op_origin_id, op_owner_id, op_name, op_ident, op_checksum, op_manifest, op_config, op_target, op_deps, op_tdeps, op_exposes)
                                RETURNING * into inserted_package;

                         SELECT id FROM origin_channels WHERE origin_id = op_origin_id AND name = 'unstable' INTO channel_id;
                         PERFORM promote_origin_package_v1(channel_id, inserted_package.id);

                         RETURN NEXT inserted_package;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_package_v1 (
                    op_ident text
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_packages WHERE ident = op_ident;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION get_origin_package_latest_v1 (
                    op_ident text,
                    op_target text
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_packages WHERE ident LIKE (op_ident  || '/%') AND target = op_target;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_package_versions_for_origin_v1 (
                    op_origin text,
                    op_pkg text
                 ) RETURNS TABLE(version text, release_count bigint) AS $$
                    BEGIN
                        RETURN QUERY SELECT p.partial_ident[3] AS version, COUNT(p.partial_ident[4]) AS release_count FROM (SELECT regexp_split_to_array(op.ident, '/') AS partial_ident FROM origin_packages op INNER JOIN origins o ON o.id = op.origin_id WHERE o.name = op_origin AND op.name = op_pkg) AS p GROUP BY version;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_v1 (
                    op_ident text,
                    op_limit bigint,
                    op_offset bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, origin_packages.ident FROM origin_packages WHERE origin_packages.ident LIKE (op_ident  || '%')
                          ORDER BY ident ASC
                          LIMIT op_limit OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_distinct_v1 (
                    op_ident text,
                    op_limit bigint,
                    op_offset bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(p.partial_ident[1] || '/' || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || '/' || p.partial_ident[2] AS ident
                        FROM (SELECT regexp_split_to_array(op.ident, '/') as partial_ident FROM origin_packages op WHERE op.ident LIKE ('%' || op_ident || '%')) AS p
                        GROUP BY (p.partial_ident[1] || '/' || p.partial_ident[2])
                        LIMIT op_limit OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_unique_for_origin_v1 (
                   op_origin text,
                   op_limit bigint,
                   op_offset bigint
                 ) RETURNS TABLE(total_count bigint, name text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, origin_packages.name FROM origins INNER JOIN origin_packages ON origins.id = origin_packages.origin_id WHERE origins.name = op_origin
                          GROUP BY origin_packages.name
                          ORDER BY origin_packages.name ASC
                          LIMIT op_limit OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_origin_packages_for_origin_v1 (
                   op_origin text,
                   op_query text,
                   op_limit bigint,
                   op_offset bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, origin_packages.ident FROM origins INNER JOIN origin_packages ON origins.id = origin_packages.origin_id WHERE origins.name = op_origin and origin_packages.name LIKE ('%' || op_query || '%')
                          ORDER BY ident ASC
                          LIMIT op_limit OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_all_origin_packages_v1 (
                   op_query text,
                   op_limit bigint,
                   op_offset bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    DECLARE
                      schema RECORD;
                    BEGIN
                      FOR schema IN EXECUTE
                        format(
                          'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                          'shard_'
                        )
                      LOOP
                        RETURN QUERY EXECUTE
                        format('SELECT COUNT(*) OVER () AS total_count, op.ident FROM %I.origins o INNER JOIN %I.origin_packages op ON o.id = op.origin_id WHERE op.ident LIKE (%L || %L || %L) ORDER BY ident ASC LIMIT %L OFFSET %L', schema.schema_name, schema.schema_name, '%', op_query, '%', op_limit, op_offset);
                      END LOOP;
                    END;
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_origin_packages_for_origin_distinct_v1 (
                   op_origin text,
                   op_query text,
                   op_limit bigint,
                   op_offset bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(p.partial_ident[1] || '/' || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || '/' || p.partial_ident[2] AS ident
                        FROM (SELECT regexp_split_to_array(op.ident, '/') as partial_ident FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id WHERE o.name = op_origin AND op.name LIKE ('%' || op_query || '%')) AS p
                        GROUP BY (p.partial_ident[1] || '/' || p.partial_ident[2])
                        LIMIT op_limit OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_all_origin_packages_dynamic_v1 (
                    op_query text,
                    op_limit bigint,
                    op_offset bigint
                    ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    DECLARE
                      schema RECORD;
                    BEGIN
                      FOR schema IN EXECUTE
                        format(
                          'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                          'shard_'
                        )
                      LOOP
                        RETURN QUERY EXECUTE
                        format('SELECT COUNT(p.partial_ident[1] || %L || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || %L || p.partial_ident[2] AS ident FROM (SELECT regexp_split_to_array(op.ident, %L) as partial_ident FROM %I.origins o INNER JOIN %I.origin_packages op ON o.id = op.origin_id WHERE o.name LIKE (%L || %L || %L) OR op.name LIKE (%L || %L || %L)) AS p GROUP BY (p.partial_ident[1] || %L || p.partial_ident[2]) LIMIT %L OFFSET %L', '/', '/', '/', schema.schema_name, schema.schema_name, '%', op_query, '%', '%', op_query, '%', '/', op_limit, op_offset);
                      END LOOP;
                    END;
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION sync_packages_v1() RETURNS TABLE(account_id bigint, package_id bigint, package_ident text, package_deps text) AS $$
                    BEGIN
                        RETURN QUERY SELECT origin_packages.owner_id, origin_packages.id, origin_packages.ident, origin_packages.deps FROM origin_packages WHERE origin_packages.scheduler_sync = false;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION set_packages_sync_v1(in_package_id bigint) RETURNS VOID AS $$
                    BEGIN
                        UPDATE origin_packages SET scheduler_sync = true WHERE id = in_package_id;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_package_versions_for_origin_v2 (
                    op_origin text,
                    op_pkg text
                 ) RETURNS TABLE(version text, release_count bigint, latest text) AS $$
                    BEGIN
                        RETURN QUERY SELECT p.partial_ident[3] AS version, COUNT(p.partial_ident[4]) AS release_count, MAX(p.partial_ident[4]) as latest FROM (SELECT regexp_split_to_array(op.ident, '/') AS partial_ident FROM origin_packages op INNER JOIN origins o ON o.id = op.origin_id WHERE o.name = op_origin AND op.name = op_pkg) AS p GROUP BY version;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv-v3",
                 r#"CREATE OR REPLACE FUNCTION get_origin_package_latest_v2 (
                    op_ident text,
                    op_target text
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_packages WHERE ident LIKE (op_ident  || '%') AND target = op_target;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv-v4",
                     r#"CREATE OR REPLACE FUNCTION search_all_origin_packages_dynamic_v2 (
                    op_query text,
                    op_limit bigint,
                    op_offset bigint
                    ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    DECLARE
                      schema RECORD;
                    BEGIN
                      FOR schema IN EXECUTE
                        format(
                          'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                          'shard_'
                        )
                      LOOP
                        RETURN QUERY EXECUTE
                        format('SELECT COUNT(p.partial_ident[1] || %L || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || %L || p.partial_ident[2] AS ident FROM (SELECT regexp_split_to_array(op.ident, %L) as partial_ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L)) AS p GROUP BY (p.partial_ident[1] || %L || p.partial_ident[2]) LIMIT %L OFFSET %L', '/', '/', '/', schema.schema_name, '%', op_query, '%', '/', op_limit, op_offset);
                      END LOOP;
                    END;
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv-v5",
                     r#"CREATE OR REPLACE FUNCTION get_origin_package_versions_for_origin_v3 (
                    op_origin text,
                    op_pkg text
                 ) RETURNS TABLE(version text, release_count bigint, latest text) AS $$
                    BEGIN
                        RETURN QUERY SELECT p.partial_ident[3] AS version, COUNT(p.partial_ident[4]) AS release_count, MAX(p.partial_ident[4]) as latest FROM (SELECT regexp_split_to_array(op.ident, '/') AS partial_ident FROM origin_packages op INNER JOIN origins o ON o.id = op.origin_id WHERE o.name = op_origin AND op.name = op_pkg) AS p GROUP BY version ORDER BY version DESC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_package_versions_for_origin_v4 (
                          op_origin text,
                          op_pkg text
                        ) RETURNS TABLE(version text, release_count bigint, latest text, platforms text)
                        LANGUAGE SQL
                        STABLE AS $$
                          WITH packages AS (
                            SELECT *
                            FROM origin_packages op INNER JOIN origins o ON o.id = op.origin_id
                            WHERE o.name = op_origin AND op.name = op_pkg
                          ), idents AS (
                            SELECT regexp_split_to_array(ident, '/') as parts, target
                            FROM packages
                          )
                          SELECT i.parts[3] AS version,
                          COUNT(i.parts[4]) AS release_count,
                          MAX(i.parts[4]) as latest,
                          ARRAY_TO_STRING(ARRAY_AGG(DISTINCT i.target), ',')
                          FROM idents i
                          GROUP BY version
                          ORDER BY version DESC
                        $$"#)?;
    migrator.migrate("originsrv-v6",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_v2 (
                    op_ident text,
                    op_limit bigint,
                    op_offset bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, origin_packages.ident FROM origin_packages WHERE origin_packages.ident LIKE (op_ident  || '%')
                          ORDER BY ident DESC
                          LIMIT op_limit OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv", r#"
                        CREATE OR REPLACE FUNCTION get_origin_package_platforms_for_package_v1 (
                          op_ident text
                        ) RETURNS TABLE (target text)
                        LANGUAGE SQL
                        VOLATILE AS $$
                        SELECT DISTINCT target FROM origin_packages WHERE ident LIKE (op_ident || '%')
                        $$;
                     "#)?;
    migrator.migrate("originsrv",
                  r#"CREATE OR REPLACE FUNCTION sync_packages_v2() RETURNS TABLE(account_id bigint, package_id bigint, package_ident text, package_deps text, package_target text) AS $$
                        SELECT owner_id, id, ident, deps, target FROM origin_packages WHERE scheduler_sync = false;
                     $$ LANGUAGE SQL STABLE"#)?;
    migrator.migrate(
        "originsrv",
        r#"UPDATE origin_packages SET scheduler_sync = false"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"ALTER TABLE IF EXISTS origin_packages ADD COLUMN IF NOT EXISTS visibility text NOT NULL DEFAULT 'public';"#,
    )?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_origin_package_v2 (
                    op_origin_id bigint,
                    op_owner_id bigint,
                    op_name text,
                    op_ident text,
                    op_checksum text,
                    op_manifest text,
                    op_config text,
                    op_target text,
                    op_deps text,
                    op_tdeps text,
                    op_exposes text
                 ) RETURNS SETOF origin_packages AS $$
                     DECLARE
                        inserted_package origin_packages;
                        channel_id bigint;
                     BEGIN
                         INSERT INTO origin_packages (origin_id, owner_id, name, ident, checksum, manifest, config, target, deps, tdeps, exposes, visibility)
                                VALUES (op_origin_id, op_owner_id, op_name, op_ident, op_checksum, op_manifest, op_config, op_target, op_deps, op_tdeps, op_exposes,
                                (SELECT default_package_visibility FROM origins WHERE id = op_origin_id))
                                RETURNING * into inserted_package;

                         SELECT id FROM origin_channels WHERE origin_id = op_origin_id AND name = 'unstable' INTO channel_id;
                         PERFORM promote_origin_package_v1(channel_id, inserted_package.id);

                         RETURN NEXT inserted_package;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_v3 (
                    op_ident text,
                    op_limit bigint,
                    op_offset bigint,
                    op_account_id bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
                          FROM origin_packages op
                          WHERE op.ident LIKE (op_ident  || '%')
                          AND (op.visibility='public' OR (op.visibility='private' AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
                          ORDER BY op.ident DESC
                          LIMIT op_limit
                          OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_for_origin_distinct_v2 (
                    op_ident text,
                    op_limit bigint,
                    op_offset bigint,
                    op_account_id bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(p.partial_ident[1] || '/' || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || '/' || p.partial_ident[2] AS ident
                        FROM (SELECT regexp_split_to_array(op.ident, '/') as partial_ident
                              FROM origin_packages op
                              WHERE op.ident LIKE ('%' || op_ident || '%')
                              AND (op.visibility='public' OR (op.visibility='private' AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
                              ) AS p
                        GROUP BY (p.partial_ident[1] || '/' || p.partial_ident[2])
                        LIMIT op_limit
                        OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION get_origin_package_latest_v3 (
                    op_ident text,
                    op_target text,
                    op_account_id bigint
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT *
                        FROM origin_packages
                        WHERE ident LIKE (op_ident  || '%')
                        AND target = op_target
                        AND (visibility='public' OR (visibility='private' AND origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)));
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_package_v2 (
                    op_ident text,
                    op_account_id bigint
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT *
                        FROM origin_packages
                        WHERE ident = op_ident
                        AND (visibility='public' OR (visibility='private' AND origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)));
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_all_origin_packages_dynamic_v3 (
                    op_query text,
                    op_limit bigint,
                    op_offset bigint,
                    op_account_id bigint
                    ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    DECLARE
                      schema RECORD;
                    BEGIN
                      FOR schema IN EXECUTE
                        format(
                          'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                          'shard_'
                        )
                      LOOP
                        RETURN QUERY EXECUTE
                        format('SELECT COUNT(p.partial_ident[1] || %L || p.partial_ident[2]) OVER () AS total_count, p.partial_ident[1] || %L || p.partial_ident[2] AS ident FROM (SELECT regexp_split_to_array(op.ident, %L) as partial_ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility=%L OR (op.visibility=%L AND op.origin_id IN (SELECT origin_id FROM %I.origin_members WHERE account_id = %L)))) AS p GROUP BY (p.partial_ident[1] || %L || p.partial_ident[2]) LIMIT %L OFFSET %L', '/', '/', '/', schema.schema_name, '%', op_query, '%', 'public', 'private', schema.schema_name, op_account_id, '/', op_limit, op_offset);
                      END LOOP;
                      RETURN;
                    END;
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_all_origin_packages_v2 (
                   op_query text,
                   op_limit bigint,
                   op_offset bigint,
                   op_account_id bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    DECLARE
                      schema RECORD;
                    BEGIN
                      FOR schema IN EXECUTE
                        format(
                          'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                          'shard_'
                        )
                      LOOP
                        RETURN QUERY EXECUTE
                        format('SELECT COUNT(*) OVER () AS total_count, op.ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility=%L OR (op.visibility=%L AND op.origin_id IN (SELECT origin_id FROM %I.origin_members WHERE account_id = %L))) ORDER BY op.ident ASC LIMIT %L OFFSET %L', schema.schema_name, '%', op_query, '%', 'public', 'private', schema.schema_name, op_account_id, op_limit, op_offset);
                      END LOOP;
                      RETURN;
                    END;
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION search_origin_packages_for_origin_v2 (
                   op_origin text,
                   op_query text,
                   op_limit bigint,
                   op_offset bigint,
                   op_account_id bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
                          FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id
                          WHERE o.name = op_origin
                          AND op.name LIKE ('%' || op_query || '%')
                          AND (op.visibility='public' OR (op.visibility='private' AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
                          ORDER BY op.ident ASC
                          LIMIT op_limit
                          OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_unique_for_origin_v2 (
                   op_origin text,
                   op_limit bigint,
                   op_offset bigint,
                   op_account_id bigint
                 ) RETURNS TABLE(total_count bigint, name text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.name
                          FROM origins o INNER JOIN origin_packages op ON o.id = op.origin_id
                          WHERE o.name = op_origin
                          AND (op.visibility='public' OR (op.visibility='private' AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
                          GROUP BY op.name
                          ORDER BY op.name ASC
                          LIMIT op_limit
                          OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_package_versions_for_origin_v5 (
                          op_origin text,
                          op_pkg text,
                          op_account_id bigint
                        ) RETURNS TABLE(version text, release_count bigint, latest text, platforms text)
                        LANGUAGE SQL
                        STABLE AS $$
                          WITH packages AS (
                            SELECT *
                            FROM origin_packages op INNER JOIN origins o ON o.id = op.origin_id
                            WHERE o.name = op_origin
                            AND op.name = op_pkg
                            AND (op.visibility='public' OR (op.visibility='private' AND op.origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
                          ), idents AS (
                            SELECT regexp_split_to_array(ident, '/') as parts, target
                            FROM packages
                          )
                          SELECT i.parts[3] AS version,
                          COUNT(i.parts[4]) AS release_count,
                          MAX(i.parts[4]) as latest,
                          ARRAY_TO_STRING(ARRAY_AGG(DISTINCT i.target), ',')
                          FROM idents i
                          GROUP BY version
                          ORDER BY version DESC
                        $$"#)?;
    migrator.migrate("originsrv", r#"
                        CREATE OR REPLACE FUNCTION get_origin_package_platforms_for_package_v2 (
                          op_ident text,
                          op_account_id bigint
                        ) RETURNS TABLE (target text)
                        LANGUAGE SQL
                        VOLATILE AS $$
                        SELECT DISTINCT target
                        FROM origin_packages
                        WHERE ident LIKE (op_ident || '%')
                        AND (visibility='public' OR (visibility='private' AND origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)))
                        $$;
                     "#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_all_origin_packages_dynamic_v4 (
                    op_query text,
                    op_account_id bigint
                    ) RETURNS TABLE(ident text) AS $$
                    DECLARE
                      schema RECORD;
                    BEGIN
                      FOR schema IN EXECUTE
                        format(
                          'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                          'shard_'
                        )
                      LOOP
                        RETURN QUERY EXECUTE
                        format('SELECT p.partial_ident[1] || %L || p.partial_ident[2] AS ident FROM (SELECT regexp_split_to_array(op.ident, %L) as partial_ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility=%L OR (op.visibility=%L AND op.origin_id IN (SELECT origin_id FROM %I.origin_members WHERE account_id = %L)))) AS p GROUP BY (p.partial_ident[1] || %L || p.partial_ident[2])', '/', '/', schema.schema_name, '%', op_query, '%', 'public', 'private', schema.schema_name, op_account_id, '/');
                      END LOOP;
                      RETURN;
                    END;
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION search_all_origin_packages_v3 (
                   op_query text,
                   op_account_id bigint
                 ) RETURNS TABLE(ident text) AS $$
                    DECLARE
                      schema RECORD;
                    BEGIN
                      FOR schema IN EXECUTE
                        format(
                          'SELECT schema_name FROM information_schema.schemata WHERE left(schema_name, 6) = %L',
                          'shard_'
                        )
                      LOOP
                        RETURN QUERY EXECUTE
                        format('SELECT op.ident FROM %I.origin_packages op WHERE op.ident LIKE (%L || %L || %L) AND (op.visibility=%L OR (op.visibility=%L AND op.origin_id IN (SELECT origin_id FROM %I.origin_members WHERE account_id = %L))) ORDER BY op.ident ASC', schema.schema_name, '%', op_query, '%', 'public', 'private', schema.schema_name, op_account_id);
                      END LOOP;
                      RETURN;
                    END;
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION update_origin_package_v1 (
                op_id bigint,
                op_owner_id bigint,
                op_name text,
                op_ident text,
                op_checksum text,
                op_manifest text,
                op_config text,
                op_target text,
                op_deps text,
                op_tdeps text,
                op_exposes text,
                op_visibility text
                 ) RETURNS void AS $$
                    UPDATE origin_packages SET
                        owner_id = op_owner_id,
                        name = op_name,
                        ident = op_ident,
                        checksum = op_checksum,
                        manifest = op_manifest,
                        config = op_config,
                        target = op_target,
                        deps = op_deps,
                        tdeps = op_tdeps,
                        exposes = op_exposes,
                        visibility = op_visibility,
                        scheduler_sync = false,
                        updated_at = now()
                        WHERE id = op_id;
                    $$ LANGUAGE SQL VOLATILE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_package_v3 (
                    op_ident text,
                    op_account_id bigint,
                    op_show_hidden bool
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT *
                        FROM origin_packages
                        WHERE ident = op_ident
                        AND (visibility='public' OR
                             (visibility='hidden' AND op_show_hidden = true) OR
                             (visibility='private' AND origin_id IN (SELECT origin_id FROM origin_members WHERE account_id = op_account_id)));
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    Ok(())
}
