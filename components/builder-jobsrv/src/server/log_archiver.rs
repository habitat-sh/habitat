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

use aws_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider};
use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use aws_sdk_rust::aws::s3::object::{GetObjectRequest, PutObjectRequest};
use aws_sdk_rust::aws::s3::s3client::S3Client;
use config::ArchiveCfg;
use error::{Result, Error};
use extern_url;
use hyper::client::Client as HyperClient;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Read;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::str::FromStr;
use VERSION;

// Note: the S3Client can't be cloned :/
pub struct LogArchiver {
    client: S3Client<DefaultCredentialsProvider, HyperClient>,
    bucket: String,
}

impl LogArchiver {
    pub fn new(config: ArchiveCfg) -> Result<Self> {

        let region = Region::from_str(config.region.as_str()).unwrap();
        let param_provider: Option<ParametersProvider>;
        param_provider = Some(ParametersProvider::with_parameters(config.key.unwrap(),
                                                                  config.secret.unwrap().as_str(),
                                                                  None)
                                      .unwrap());
        // If given an endpoint, don't use virtual buckets... if not,
        // assume AWS and use virtual buckets.
        //
        // There may be a way to set Minio up to use virtual buckets,
        // but I haven't been able to find it... if there is, then we
        // can go ahead and make this a configuration parameter as well.
        let use_virtual_buckets = !config.endpoint.is_some();

        // Parameterize this if anyone ends up needing V2 signatures
        let signature_type = Signature::V4;
        let final_endpoint = match config.endpoint {
            Some(url) => Some(extern_url::Url::parse(url.as_str())?),
            None => None,
        };
        let user_agent = format!("Habitat-Builder/{}", VERSION);

        let provider = DefaultCredentialsProvider::new(param_provider).unwrap();
        let endpoint = Endpoint::new(region,
                                     signature_type,
                                     final_endpoint,
                                     None,
                                     Some(user_agent),
                                     Some(use_virtual_buckets));

        let client = S3Client::new(provider, endpoint);
        Ok(LogArchiver {
               client: client,
               bucket: config.bucket.unwrap(),
           })
    }


    /// Upload the contents of the given job log to S3 for long-term
    /// storage.
    pub fn archive<T>(&self, job_id: u64, file_path: T) -> Result<()>
        where T: AsRef<Path> + Debug
    {
        let mut buffer = Vec::new();
        let mut put_object = PutObjectRequest::default();
        put_object.bucket = self.bucket.clone(); 
        put_object.key = LogArchiver::key(job_id);

        let handle = OpenOptions::new().read(true).open(&file_path);

        match handle {
            Ok(mut file) => {
                file.read_to_end(&mut buffer)?;
                put_object.body = Some(buffer.as_slice());
            }
            Err(e) => {
                // TODO: Figure out best way to handle this
                warn!("Could not open {:?} for reading; not archiving! {:?}",
                      file_path,
                      e);
                return Err(Error::from(e));
            }
        }

        // This panics if it can't resolve the URL (e.g.,
        // there's a netsplit, your Minio goes down, S3 goes down (!)).
        // We have to catch it, otherwise no more logs get captured!
        //
        // The code in the S3 library we're currently using isn't
        // UnwindSafe, so we need to deal with that, too.
        let result =
            panic::catch_unwind(AssertUnwindSafe(|| self.client.put_object(&put_object, None)));

        match result {
            Ok(Ok(_)) => Ok(()), // normal result
            Ok(Err(e)) => {
                // This is a "normal", non-panicking error, e.g.,
                // they're configured with a non-existent bucket.
                Err(Error::JobLogArchive(job_id, e))
            }, 
            Err(e) => {
                let source = match e.downcast_ref::<String>() {
                    Some(string) => string.to_string(),
                    None => format!("{:?}", e)
                };
                Err(Error::CaughtPanic(format!("Failure to archive log for job {}", job_id),
                                       source))
            }
        }
    }

    /// Given a `job_id`, retrieve the lines of the log from archival
    /// storage.
    pub fn retrieve(&self, job_id: u64) -> Result<Vec<String>> {
        let mut request = GetObjectRequest::default();
        request.bucket = self.bucket.clone();
        request.key = LogArchiver::key(job_id);

        // As above when uploading a job file, we currently need to
        // catch a potential panic if the object store cannot be reached
        let result =
            panic::catch_unwind(AssertUnwindSafe(|| self.client.get_object(&request, None)));

        let body = match result {
            Ok(Ok(response)) => response.body, // normal result
            Ok(Err(e)) => {
                // This is a "normal", non-panicking error, e.g.,
                // they're configured with a non-existent bucket.
                return Err(Error::JobLogRetrieval(job_id, e))
            }, 
            Err(e) => {
                let source = match e.downcast_ref::<String>() {
                    Some(string) => string.to_string(),
                    None => format!("{:?}", e)
                };
                return Err(Error::CaughtPanic(
                    format!("Failure to retrieve archived log for job {}", job_id),
                    source))
            }
        };

        let lines = String::from_utf8_lossy(body.as_slice())
            .lines()
            .map(|l| l.to_string())
            .collect();

        Ok(lines)
    }

    fn key(job_id: u64) -> String {
        format!("{}.log", job_id)
    }
}
