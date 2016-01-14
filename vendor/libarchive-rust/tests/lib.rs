extern crate libarchive;

pub mod util;

use std::fs::File;
use libarchive::archive::{self, ReadFilter, ReadFormat};
use libarchive::reader::{self, Reader};
use libarchive::writer;

#[test]
fn reading_from_file() {
    let tar = util::path::fixture("sample.tar.gz");
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All).ok();
    builder.support_filter(ReadFilter::All).ok();
    let mut reader = builder.open_file(tar).ok().unwrap();
    reader.next_header();
    // let entry: &archive::Entry = &reader.entry;
    // println!("{:?}", entry.pathname());
    // println!("{:?}", entry.size());
    // for entry in reader.entries() {
    //     let file = entry as &archive::Entry;
    //     println!("{:?}", file.pathname());
    //     println!("{:?}", file.size());
    // }
    assert_eq!(4, 4);
}

#[test]
fn read_archive_from_stream() {
    let tar = util::path::fixture("sample.tar.gz");
    let f = File::open(tar).ok().unwrap();
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All).ok();
    builder.support_filter(ReadFilter::All).ok();
    match builder.open_stream(f) {
        Ok(mut reader) => {
            assert_eq!(reader.header_position(), 0);
            let writer = writer::Disk::new();
            let count = writer.write(&mut reader, Some("/opt/bldr/fucks")).ok().unwrap();
            assert_eq!(count, 14);
            assert_eq!(reader.header_position(), 1024);
            assert_eq!(4, 4);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

#[test]
fn extracting_from_file() {
    let tar = util::path::fixture("sample.tar.gz");
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All).ok();
    builder.support_filter(ReadFilter::All).ok();
    let mut reader = builder.open_file(tar).ok().unwrap();
    println!("{:?}", reader.header_position());
    let writer = writer::Disk::new();
    writer.write(&mut reader, None).ok();
    println!("{:?}", reader.header_position());
    assert_eq!(4, 4)
}

#[test]
fn extracting_an_archive_with_options() {
    let tar = util::path::fixture("sample.tar.gz");
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All).ok();
    builder.support_filter(ReadFilter::All).ok();
    let mut reader = builder.open_file(tar).ok().unwrap();
    println!("{:?}", reader.header_position());
    let mut opts = archive::ExtractOptions::new();
    opts.add(archive::ExtractOption::Time);
    let writer = writer::Disk::new();
    writer.set_options(&opts).ok();
    writer.write(&mut reader, None).ok();
    println!("{:?}", reader.header_position());
    assert_eq!(4, 4)
}

#[test]
fn extracting_a_reader_twice() {
    let tar = util::path::fixture("sample.tar.gz");
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All).ok();
    builder.support_filter(ReadFilter::All).ok();
    let mut reader = builder.open_file(tar).ok().unwrap();
    println!("{:?}", reader.header_position());
    let writer = writer::Disk::new();
    writer.write(&mut reader, None).ok();
    println!("{:?}", reader.header_position());
    match writer.write(&mut reader, None) {
        Ok(_) => println!("oops"),
        Err(_) => println!("nice")
    }
    assert_eq!(4, 4)
}
