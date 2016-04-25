// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
#[macro_use]
extern crate hyper;
extern crate redis;
extern crate rustc_serialize;

pub mod data_object;

header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }
