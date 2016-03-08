// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use time;
use walkdir::WalkDir;

use super::Depot;
use super::data_object::{self, DataObject};
use super::data_store::{Cursor, Database, Transaction};
use error::{BldrError, BldrResult};
use package::{self, PackageArchive};

pub struct ReportBuilder {
    pub operations: Vec<Operation>,
    pub start: u64,
}

impl ReportBuilder {
    pub fn new() -> Self {
        ReportBuilder::default()
    }

    pub fn success(&mut self, operation: OperationType) -> &mut Self {
        self.add(Operation::Success(operation));
        self
    }

    pub fn failure(&mut self, operation: OperationType, reason: Reason) -> &mut Self {
        self.add(Operation::Failure(operation, reason));
        self
    }

    pub fn generate(self) -> Report {
        let time = time::precise_time_ns();
        Report {
            start: self.start,
            finish: time,
            success: self.operations.iter().all(Self::check_success),
            operations: self.operations,
        }
    }

    fn add(&mut self, operation: Operation) {
        self.operations.push(operation);
    }

    fn check_success(op: &Operation) -> bool {
        match *op {
            Operation::Success(_) => true,
            Operation::Failure(_, _) => false,
        }
    }
}

impl Default for ReportBuilder {
    fn default() -> Self {
        ReportBuilder {
            operations: vec![],
            start: time::precise_time_ns(),
        }
    }
}

#[derive(Debug)]
pub struct Report {
    pub start: u64,
    pub finish: u64,
    pub success: bool,
    pub operations: Vec<Operation>,
}

#[derive(Debug)]
pub enum OperationType {
    ArchiveSort(String),
    ArchiveInsert(String),
    CleanupTrash(String),
    InitDepotFs(String),
    Trashed(String, Reason),
    TruncateDatabase(&'static str, usize),
}

#[derive(Debug)]
pub enum Reason {
    BadArchive,
    BadMetadata(BldrError),
    BadPermissions,
    FileExists,
    NotEmpty,
}

#[derive(Debug)]
pub enum Operation {
    Success(OperationType),
    Failure(OperationType, Reason),
}

struct Doctor<'a> {
    report: ReportBuilder,
    depot: &'a Depot,
    packages_path: PathBuf,
}

impl<'a> Doctor<'a> {
    pub fn new(depot: &'a Depot) -> Self {
        let report = ReportBuilder::new();
        let mut packages = depot.packages_path().clone();
        packages.pop();
        packages.push(format!("pkgs.{:?}", report.start));
        Doctor {
            report: report,
            depot: depot,
            packages_path: packages,
        }
    }

    fn run(mut self) -> BldrResult<Report> {
        try!(self.init_fs());
        try!(self.truncate_database(&self.depot.datastore.packages));
        try!(self.sort_packages());
        try!(self.rebuild_metadata());
        Ok(self.report.generate())
    }

    fn init_fs(&mut self) -> BldrResult<()> {
        match fs::metadata(&self.depot.path) {
            Ok(meta) => {
                if meta.is_file() {
                    self.report.failure(OperationType::InitDepotFs(self.depot.path.clone()),
                                        Reason::FileExists);
                }
                if meta.permissions().readonly() {
                    self.report.failure(OperationType::InitDepotFs(self.depot.path.clone()),
                                        Reason::BadPermissions);
                }
                try!(fs::create_dir_all(&self.depot.packages_path()));
            }
            Err(_) => try!(fs::create_dir_all(&self.depot.packages_path())),
        }
        try!(fs::rename(&self.depot.packages_path(), &self.packages_path));
        try!(fs::create_dir_all(&self.depot.packages_path()));
        Ok(())
    }

    fn rebuild_metadata(&mut self) -> BldrResult<()> {
        let txn = try!(self.depot.datastore.packages.txn_rw());
        for entry in WalkDir::new(&self.depot.packages_path()).follow_links(false) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                continue;
            }
            let mut archive = PackageArchive::new(PathBuf::from(entry.path()));
            match data_object::Package::from_archive(&mut archive) {
                Ok(object) => {
                    try!(self.depot.datastore.packages.write(&txn, &object));
                }
                Err(e) => {
                    // We should be moving this back to the garbage directory and recording the
                    // path of it there in this failure
                    self.report.failure(OperationType::ArchiveInsert(entry.path()
                                                                          .to_string_lossy()
                                                                          .to_string()),
                                        Reason::BadMetadata(e));
                }
            }
        }
        try!(txn.commit());
        let txn = try!(self.depot.datastore.views.pkg_view_idx.txn_rw());
        {
            let tx2 = try!(txn.new_child_rw(&self.depot.datastore.views.view_pkg_idx));
            try!(tx2.clear());
        }
        let mut cursor = try!(txn.cursor_ro());
        loop {
            match cursor.next_nodup() {
                Ok((key, view)) => {
                    match package::PackageIdent::from_str(&key) {
                        Ok(pkident) => {
                            let ident = data_object::PackageIdent::new(pkident);
                            let path = self.depot.archive_path(&ident);
                            match fs::metadata(&path) {
                                Ok(_) => {
                                    let tx2 = try!(txn.new_child_rw(&self.depot
                                                                         .datastore
                                                                         .views
                                                                         .view_pkg_idx));
                                    try!(tx2.put(view.ident(), &ident));
                                    loop {
                                        match cursor.next_dup() {
                                            Ok((_, view)) => {
                                                try!(tx2.put(view.ident(), &ident));
                                            }
                                            Err(_) => break,
                                        }
                                    }
                                }
                                Err(_) => try!(txn.delete(&key, None)),
                            }
                        }
                        Err(_) => try!(txn.delete(&key, None)),
                    }
                }
                Err(_) => break,
            }
        }
        Ok(())
    }

    fn sort_packages(&mut self) -> BldrResult<()> {
        let mut directories = vec![];
        for entry in WalkDir::new(&self.packages_path).follow_links(false) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                directories.push(entry);
                continue;
            }
            let mut archive = PackageArchive::new(PathBuf::from(entry.path()));
            match archive.ident() {
                Ok(ident) => {
                    let path = self.depot.archive_path(&ident);
                    try!(fs::create_dir_all(path.parent().unwrap()));
                    try!(fs::rename(entry.path(), path));
                    self.report
                        .success(OperationType::ArchiveSort(entry.path()
                                                                 .to_string_lossy()
                                                                 .to_string()));
                }
                Err(e) => {
                    debug!("Error reading, archive={:?} error={:?}", &archive, &e);
                    self.report.failure(OperationType::ArchiveSort(entry.path()
                                                                        .to_string_lossy()
                                                                        .to_string()),
                                        Reason::BadArchive);
                }
            }
        }
        directories.reverse();
        for dir in directories.iter() {
            if let Some(e) = fs::remove_dir(dir.path()).err() {
                debug!("Error deleting: {:?}", &e);
                self.report.failure(OperationType::CleanupTrash(self.packages_path
                                                                    .to_string_lossy()
                                                                    .to_string()),
                                    Reason::NotEmpty);
            }
        }
        Ok(())
    }

    fn truncate_database<D: Database>(&mut self, database: &D) -> BldrResult<()> {
        let count = {
            let txn = try!(database.txn_rw());
            let stats = try!(database.stat(&txn));
            try!(database.clear(&txn));
            stats.entries()
        };
        self.report.success(OperationType::TruncateDatabase(D::name(), count));
        Ok(())
    }
}

pub fn repair(depot: &Depot) -> BldrResult<Report> {
    Doctor::new(depot).run()
}
