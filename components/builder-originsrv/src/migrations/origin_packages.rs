// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use error::Result;

pub fn migrate(migrator: &mut Migrator) -> Result<()> {
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
                    ) RETURNS TABLE (
                        id bigint,
                        name text,
                        origin_id bigint,
                        owner_id bigint,
                        ident text,
                        checksum text,
                        manifest text,
                        config text,
                        target text,
                        deps text,
                        tdeps text,
                        exposes text,
                        channels text[]
                    ) AS $$
                        DECLARE
                            channel_id bigint;
                            package_id bigint;
                        BEGIN
                            INSERT INTO origin_packages (origin_id, owner_id, name, ident, checksum, manifest, config, target, deps, tdeps, exposes)
                            VALUES (op_origin_id, op_owner_id, op_name, op_ident, op_checksum, op_manifest, op_config, op_target, op_deps, op_tdeps, op_exposes)
                            RETURNING origin_packages.id INTO package_id;

                            SELECT oc.id FROM origin_channels oc WHERE oc.origin_id = op_origin_id AND oc.name = 'unstable'
                              INTO channel_id;

                            PERFORM promote_origin_package_v1(channel_id, package_id);

                            RETURN QUERY
                                SELECT p.id, p.name, p.origin_id, p.owner_id, p.ident, p.checksum, p.manifest,
                                       p.config, p.target, p.deps, p.tdeps, p.exposes,
                                    (SELECT ARRAY(SELECT c.name
                                                    FROM origin_channels c
                                                   WHERE c.id IN (SELECT cp.channel_id
                                                                    FROM origin_channel_packages cp
                                                                   WHERE cp.package_id = p.id
                                                                ORDER BY cp.created_at DESC))) AS channels
                                  FROM origin_packages p
                                 WHERE p.id = package_id;
                            RETURN;
                        END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
         r#"CREATE OR REPLACE FUNCTION get_origin_package_v2 (
                op_ident text
            ) RETURNS TABLE (
                id bigint,
                name text,
                origin_id bigint,
                owner_id bigint,
                ident text,
                checksum text,
                manifest text,
                config text,
                target text,
                deps text,
                tdeps text,
                exposes text,
                channels text[]
            ) AS $$
                BEGIN
                    RETURN QUERY
                        SELECT p.id, p.name, p.origin_id, p.owner_id, p.ident, p.checksum, p.manifest,
                            p.config, p.target, p.deps, p.tdeps, p.exposes,
                            (SELECT ARRAY(SELECT c.name
                               FROM origin_channels c
                              WHERE c.id IN (SELECT channel_id
                                               FROM origin_channel_packages
                                              WHERE package_id = p.id
                                              ORDER BY created_at DESC))) AS channels
                         FROM origin_packages p
                        WHERE p.ident = op_ident;
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
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_packages_for_version_v1 (
                            op_ident text,
                            op_limit bigint,
                            op_offset bigint
                        ) RETURNS TABLE(total_count bigint, ident text, channels text[]) AS
                        $$
                            BEGIN
                                RETURN QUERY
                                   SELECT COUNT(*) OVER ()
                                       AS total_count,
                                          origin_packages.ident,
                                          (SELECT ARRAY(SELECT name
                                                          FROM origin_channels
                                                         WHERE id IN (SELECT channel_id
                                                                        FROM origin_channel_packages
                                                                       WHERE package_id = origin_packages.id
                                                                       ORDER BY created_at DESC)))
                                     FROM origin_packages
                                    WHERE origin_packages.ident LIKE (op_ident  || '%')
                                 ORDER BY ident DESC
                                    LIMIT op_limit
                                   OFFSET op_offset;
                                RETURN;
                            END
                        $$
                        LANGUAGE plpgsql STABLE"#)?;
    Ok(())
}
