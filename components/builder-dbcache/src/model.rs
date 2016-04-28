// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use rustc_serialize::{Decodable, Encodable};

use data_store::{DataRecord, InstaId, Table};

pub type Fields = Vec<(&'static str, String)>;

pub trait Model: Encodable + Decodable + From<DataRecord> {
    type Table: Table;

    fn fields(&self) -> Fields;
    fn id(&self) -> &InstaId;
    fn set_id(&mut self, InstaId) -> ();
}
