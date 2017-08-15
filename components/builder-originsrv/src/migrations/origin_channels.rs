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
        r#"CREATE SEQUENCE IF NOT EXISTS origin_channel_id_seq;"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_channels (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_channel_id_seq'),
                    origin_id bigint REFERENCES origins(id),
                    owner_id bigint,
                    name text,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz,
                    UNIQUE(origin_id, name)
             )"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE TABLE IF NOT EXISTS origin_channel_packages (
                    channel_id bigint REFERENCES origin_channels(id) ON DELETE CASCADE,
                    package_id bigint REFERENCES origin_packages(id),
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz,
                    PRIMARY KEY (channel_id, package_id)
             )"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION insert_origin_channel_v1 (
                    occ_origin_id bigint,
                    occ_owner_id bigint,
                    occ_name text
                 ) RETURNS SETOF origin_channels AS $$
                     BEGIN
                         RETURN QUERY INSERT INTO origin_channels (origin_id, owner_id, name)
                                VALUES (occ_origin_id, occ_owner_id, occ_name)
                                RETURNING *;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#,
    )?;
    migrator
        .migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION get_origin_channel_v1 (
                    ocg_origin text,
                    ocg_name text
                 ) RETURNS SETOF origin_channels AS $$
                    BEGIN
                        RETURN QUERY SELECT origin_channels.*
                          FROM origins INNER JOIN origin_channels ON origins.id = origin_channels.origin_id
                          WHERE origins.name=ocg_origin AND origin_channels.name = ocg_name;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_channels_for_origin_v1 (
                   occ_origin_id bigint
                 ) RETURNS SETOF origin_channels AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_channels WHERE origin_id = occ_origin_id
                          ORDER BY name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION promote_origin_package_v1 (
                    opp_channel_id bigint,
                    opp_package_id bigint
                 ) RETURNS void AS $$
                        INSERT INTO origin_channel_packages (channel_id, package_id) VALUES (opp_channel_id, opp_package_id)
                        ON CONFLICT ON CONSTRAINT origin_channel_packages_pkey DO NOTHING;
                 $$ LANGUAGE SQL VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION promote_origin_package_group_v1 (
                    opp_channel_id bigint,
                    opp_package_ids bigint[]
                 ) RETURNS void
                   LANGUAGE SQL
                   VOLATILE AS $$
                     INSERT INTO origin_channel_packages (channel_id, package_id)
                     SELECT opp_channel_id, package_ids.id
                     FROM unnest(opp_package_ids) AS package_ids(id)
                     ON CONFLICT ON CONSTRAINT origin_channel_packages_pkey DO NOTHING;
                 $$"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION demote_origin_package_v1 (
                    opp_channel_id bigint,
                    opp_package_id bigint
                 ) RETURNS void AS $$
                        DELETE FROM origin_channel_packages WHERE channel_id=opp_channel_id AND package_id=opp_package_id;
                 $$ LANGUAGE SQL VOLATILE"#)?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION delete_origin_channel_v1 (
                    channel_id bigint
                 ) RETURNS void AS $$
                    BEGIN
                        DELETE FROM origin_channels WHERE id = channel_id;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#,
    )?;

    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_channel_package_v1 (
                    op_origin text,
                    op_channel text,
                    op_ident text
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT op.*
                          FROM origin_packages op
                          INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
                          INNER JOIN origin_channels oc on ocp.channel_id = oc.id
                          INNER JOIN origins o on oc.origin_id = o.id
                          WHERE op.ident = op_ident AND o.name = op_origin AND oc.name = op_channel;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_channel_package_latest_v1 (
                    op_origin text,
                    op_channel text,
                    op_ident text,
                    op_target text
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT op.*
                          FROM origin_packages op
                          INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
                          INNER JOIN origin_channels oc on ocp.channel_id = oc.id
                          INNER JOIN origins o on oc.origin_id = o.id
                          WHERE o.name = op_origin
                          AND oc.name = op_channel
                          AND op.ident LIKE (op_ident  || '%')
                          AND op.target = op_target
                          ORDER BY op.ident DESC
                          LIMIT 1;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate(
        "originsrv",
        r#"CREATE OR REPLACE FUNCTION get_origin_channel_packages_for_channel_v1 (
                    op_origin text,
                    op_channel text,
                    op_ident text,
                    op_limit bigint,
                    op_offset bigint
                 ) RETURNS TABLE(total_count bigint, ident text) AS $$
                    BEGIN
                        RETURN QUERY SELECT COUNT(*) OVER () AS total_count, op.ident
                          FROM origin_packages op
                          INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
                          INNER JOIN origin_channels oc on ocp.channel_id = oc.id
                          INNER JOIN origins o on oc.origin_id = o.id
                          WHERE o.name = op_origin
                          AND oc.name = op_channel
                          AND op.ident LIKE (op_ident  || '%')
                          ORDER BY ident ASC
                          LIMIT op_limit OFFSET op_offset;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate(
        "originsrv-v2",
        r#"CREATE OR REPLACE FUNCTION get_origin_channel_package_latest_v2 (
                    op_origin text,
                    op_channel text,
                    op_ident text,
                    op_target text
                 ) RETURNS SETOF origin_packages AS $$
                    BEGIN
                        RETURN QUERY SELECT op.*
                          FROM origin_packages op
                          INNER JOIN origin_channel_packages ocp on ocp.package_id = op.id
                          INNER JOIN origin_channels oc on ocp.channel_id = oc.id
                          INNER JOIN origins o on oc.origin_id = o.id
                          WHERE o.name = op_origin
                          AND oc.name = op_channel
                          AND op.ident LIKE (op_ident  || '%')
                          AND op.target = op_target;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
    )?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION get_origin_package_channels_for_package_v1 (
                    op_ident text
                 ) RETURNS SETOF origin_channels AS $$
                    BEGIN
                        RETURN QUERY SELECT oc.*
                            FROM origin_channels oc INNER JOIN origin_channel_packages ocp ON oc.id = ocp.channel_id
                            INNER JOIN origin_packages op ON op.id = ocp.package_id
                            WHERE op.ident=op_ident
                            ORDER BY oc.name;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    Ok(())
}
