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
    migrator
        .migrate("originsrv",
                 r#"CREATE SEQUENCE IF NOT EXISTS origin_project_id_seq;"#)?;
    migrator
        .migrate("originsrv",
                 r#"CREATE TABLE IF NOT EXISTS origin_projects (
                        id bigint PRIMARY KEY DEFAULT next_id_v1('origin_project_id_seq'),
                        origin_id bigint REFERENCES origins(id),
                        origin_name text,
                        package_name text,
                        name text,
                        plan_path text,
                        owner_id bigint,
                        vcs_type text,
                        vcs_data text,
                        created_at timestamptz DEFAULT now(),
                        updated_at timestamptz,
                        UNIQUE (origin_name, package_name, name)
                        )"#)?;
    migrator
        .migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION insert_origin_project_v1 (
                        project_origin_name text,
                        project_package_name text,
                        project_plan_path text,
                        project_vcs_type text,
                        project_vcs_data text,
                        project_owner_id bigint
                 ) RETURNS SETOF origin_projects AS $$
                     BEGIN
                         RETURN QUERY INSERT INTO origin_projects (origin_id,
                                                      origin_name,
                                                      package_name,
                                                      name,
                                                      plan_path,
                                                      owner_id,
                                                      vcs_type,
                                                      vcs_data)
                                VALUES (
                                    (SELECT id FROM origins where name = project_origin_name),
                                    project_origin_name,
                                    project_package_name,
                                    project_origin_name || '/' || project_package_name,
                                    project_plan_path,
                                    project_owner_id,
                                    project_vcs_type,
                                    project_vcs_data)
                                RETURNING *;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator
        .migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION get_origin_project_v1 (
                    project_name text
                 ) RETURNS SETOF origin_projects AS $$
                    BEGIN
                        RETURN QUERY SELECT * FROM origin_projects WHERE name = project_name;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#)?;
    migrator
        .migrate("originsrv",
                 r#"CREATE OR REPLACE FUNCTION delete_origin_project_v1 (
                    project_name text
                 ) RETURNS void AS $$
                    BEGIN
                        DELETE FROM origin_projects WHERE name = project_name;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#)?;
    migrator.migrate("originsrv",
                     r#"CREATE OR REPLACE FUNCTION update_origin_project_v1 (
                        project_id bigint,
                        project_origin_id bigint,
                        project_package_name text,
                        project_plan_path text,
                        project_vcs_type text,
                        project_vcs_data text,
                        project_owner_id bigint
                 ) RETURNS void AS $$
                     BEGIN
                        UPDATE origin_projects SET
                            package_name = project_package_name,
                            name = (SELECT name FROM origins WHERE id = project_origin_id) || '/' || project_package_name,
                            plan_path = project_plan_path,
                            vcs_type = project_vcs_type,
                            vcs_data = project_vcs_data,
                            owner_id = project_owner_id,
                            updated_at = now()
                            WHERE id = project_id;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#)?;
    Ok(())
}
