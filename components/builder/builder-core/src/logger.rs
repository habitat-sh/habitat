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

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;
use chrono::prelude::*;

use protocol::jobsrv::{Job, JobGroup, JobGroupProject};

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn init<T, U>(log_path: T, filename: U) -> Self
    where
        T: AsRef<Path>,
        U: AsRef<Path>,
    {
        Logger {
            file: OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_path.as_ref().join(filename))
                .expect("Failed to initialize log file"),
        }
    }

    pub fn log(&mut self, msg: &str) {
        let dt: DateTime<Utc> = Utc::now();
        let fmt_msg = format!("{},{}\n", dt.format("%Y-%m-%d %H:%M:%S"), msg);

        self.file.write_all(fmt_msg.as_bytes()).expect(&format!(
            "Logger unable to write to {:?}",
            self.file
        ));
    }

    // Log format (fields are comma-separated)
    //   Log entry datetime (Utc)
    //   Entry type - G (group), J (job), P (project), W (worker), I (ident)
    //   Id (group or job id)
    //   State
    //   Project name (for job or project)
    //   Start datetime (Utc) (only for jobs)
    //   End datetime (Utc) (only for jobs)
    //   Start offset (offset from group creation, in seconds, only for jobs)
    //   Duration (job duration, in seconds, only for jobs)
    //   Error (if applicable)
    pub fn log_ident(&mut self, ident: &str) {
        let msg = format!("I,Started {}", ident);
        self.log(&msg);
    }

    pub fn log_group(&mut self, group: &JobGroup) {
        let msg = format!("G,{},{:?},", group.get_id(), group.get_state());
        self.log(&msg);
    }

    pub fn log_group_project(&mut self, group: &JobGroup, project: &JobGroupProject) {
        let msg = format!(
            "P,{},{:?},{},",
            group.get_id(),
            project.get_state(),
            project.get_name()
        );
        self.log(&msg);
    }

    pub fn log_group_job(&mut self, group: &JobGroup, job: &Job) {
        let suffix = if job.has_build_started_at() && job.has_build_finished_at() {
            let start = job.get_build_started_at().parse::<DateTime<Utc>>().unwrap();
            let stop = job.get_build_finished_at()
                .parse::<DateTime<Utc>>()
                .unwrap();
            let group_start = group.get_created_at().parse::<DateTime<Utc>>().unwrap();

            let offset = start
                .signed_duration_since(group_start)
                .to_std()
                .unwrap()
                .as_secs();
            let duration = stop.signed_duration_since(start)
                .to_std()
                .unwrap()
                .as_secs();

            format!(
                "{},{},{},{}",
                offset,
                duration,
                start.format("%Y-%m-%d %H:%M:%S").to_string(),
                stop.format("%Y-%m-%d %H:%M:%S").to_string()
            )
        } else {
            "".to_string()
        };

        let error = if job.has_error() {
            format!("{:?}", job.get_error())
        } else {
            "".to_string()
        };

        let msg = format!(
            "J,{},{},{:?},{},{},{}",
            job.get_owner_id(),
            job.get_id(),
            job.get_state(),
            job.get_project().get_name(),
            suffix,
            error
        );

        self.log(&msg);
    }

    pub fn log_worker_job(&mut self, job: &Job) {
        let start = if job.has_build_started_at() {
            job.get_build_started_at()
                .parse::<DateTime<Utc>>()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        } else {
            "".to_string()
        };

        let stop = if job.has_build_finished_at() {
            job.get_build_finished_at()
                .parse::<DateTime<Utc>>()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        } else {
            "".to_string()
        };

        let msg = format!(
            "W,{},{},{:?},{},,,{},{},{:?}",
            job.get_owner_id(),
            job.get_id(),
            job.get_state(),
            job.get_project().get_name(),
            start,
            stop,
            job.get_error()
        );
        self.log(&msg);
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.file.sync_all().expect("Unable to sync log file");
    }
}
