// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use db::pool::Pool;
use db::migration::Migrator;
use db::error::{Result, Error};

#[derive(Debug)]
struct MusicDB {
    pool: Pool,
}

impl MusicDB {
    fn new(pool: Pool) -> MusicDB {
        MusicDB { pool: pool }
    }

    fn setup(&self) -> Result<()> {
        let mut migrator = Migrator::new(&self.pool);
        migrator.setup()?;
        migrator.migrate("music",
                     r#"CREATE TABLE IF NOT EXISTS music (
            band text PRIMARY KEY,
            style text,
            created_at timestamptz DEFAULT now(),
            updated_at timestamptz
        )"#)?;
        migrator.migrate("music",
                     r#"CREATE VIEW metal_bands AS 
            SELECT band FROM music WHERE style = 'metal'"#)?;
        migrator.migrate("music",
                     r#"CREATE OR REPLACE FUNCTION insert_band_v1(band text, style text) RETURNS void AS $$
                         BEGIN
                            INSERT INTO music (band, style) VALUES (band, style);
                         END
                         $$ LANGUAGE plpgsql VOLATILE"#)?;
        Ok(())
    }

    fn insert_band(&self, band: &str, style: &str) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute("SELECT insert_band_v1($1, $2)", &[&band, &style])
            .map_err(Error::FunctionRun)?;
        Ok(())
    }

    fn get_metal_bands(&self) -> Result<Vec<String>> {
        let conn = self.pool.get()?;
        let results = conn.query("SELECT band FROM metal_bands", &[]).map_err(Error::FunctionRun)?;
        Ok(results.into_iter().map(|r| r.get(0)).collect())
    }
}

#[test]
fn create() {
    with_pool!(pool, {
        let mdb = MusicDB::new(pool);
        mdb.setup().expect("Failed to migrate the music database");
    });
}

#[test]
fn insert() {
    with_pool!(pool, {
        let mdb = MusicDB::new(pool);
        mdb.setup().expect("Failed to migrate the music database");
        mdb.insert_band("katatonia", "metal").expect("Failed to migrate the music database");
        mdb.insert_band("nirvana", "grunge").expect("Failed to migrate the music database");
    });
}

#[test]
fn query() {
    with_pool!(pool, {
        let mdb = MusicDB::new(pool);
        mdb.setup().expect("Failed to migrate the music database");
        mdb.insert_band("katatonia", "metal").expect("Failed to migrate the music database");
        mdb.insert_band("nirvana", "grunge").expect("Failed to migrate the music database");
        let metal_bands = mdb.get_metal_bands().expect("Failed to get metal bands");
        assert!(metal_bands.contains(&String::from("katatonia")),
                "There are no metal bands present, and katatonia is certainly a metal band");
    });
}
