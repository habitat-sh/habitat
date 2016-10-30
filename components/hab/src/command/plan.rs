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

pub mod initialize {
    use std::fs::create_dir_all;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use common::ui::{UI, Status};
    use error::Result;

    const PLAN_TEMPLATE: &'static [u8] = b"test plan.sh";
    const RUN_HOOK_TEMPLATE: &'static [u8] = b"test run hook";

    pub fn start(ui: &mut UI) -> Result<()> {
        try!(ui.begin("Constructing a cozy habitat for your app..."));
        try!(ui.br());

        try!(create_with_template(ui, "habitat/plan.sh", PLAN_TEMPLATE));
        try!(ui.para("The `plan.sh` is the foundation of your new habitat. You can \
            define core metadata, dependencies, and tasks. More documentation here: TODO"));

        try!(create_with_template(ui, "habitat/hooks/run", RUN_HOOK_TEMPLATE));
        try!(ui.para("The `hooks` directory is where you can create a number of automation hooks \
            into your habitat. We'll make a `run` hook to get you started, but there are more \
            hooks to create and tweak! See the full list with info here: TODO"));

        try!(ui.end("A happy abode for your code has been initialized! Now it's time to explore!"));
        Ok(())
    }

    fn create_with_template(ui: &mut UI, location: &str, template:  &[u8]) -> Result<()> {
        let path = Path::new(&location);
        match path.exists() {
            false => {
                try!(ui.status(Status::Creating, format!("file: {}", location)));
                if let Some(directory) = path.parent() {
                    try!(create_dir_all(directory));
                }
                try!(File::create(path).and_then(|mut file| file.write(template)));
            },
            true => {
                try!(ui.status(Status::Using, format!("existing file: {}", location)));
            }
        };
        Ok(())
    }
}
