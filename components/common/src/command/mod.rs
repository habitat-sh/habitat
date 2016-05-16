// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod package;

use std::io::{self, Write};

use pbr;
use depot_client::DisplayProgress;

/// A moving progress bar to track progress of a sized event, similar to wget, curl, npm, etc.
///
/// This is designed to satisfy a generic behavior which sets the size of the task (usually a
/// number of bytes representing the total download/upload/transfer size) and will be a generic
/// writer (i.e. implementing the `Write` trait) as a means to increase progress towards
/// completion.
pub struct ProgressBar {
    bar: pbr::ProgressBar,
}

impl Default for ProgressBar {
    fn default() -> Self {
        ProgressBar { bar: pbr::ProgressBar::new(0) }
    }
}

impl DisplayProgress for ProgressBar {
    fn size(&mut self, size: u64) {
        self.bar = pbr::ProgressBar::new(size);
        self.bar.set_units(pbr::Units::Bytes);
        self.bar.show_tick = true;
        self.bar.message("    ");
    }
}

impl Write for ProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.bar.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.bar.flush()
    }
}
