use crate::{api_client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use std::io::Write;
use tabwriter::TabWriter;

pub fn start(ui: &mut UI,
             bldr_url: &str,
             group_id: Option<&str>,
             origin: Option<&str>,
             limit: usize,
             show_jobs: bool)
             -> Result<()> {
    let api_client =
        api_client::Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    if let Some(o) = origin {
        do_origin_status(ui, &api_client, o, limit)?;
    } else {
        do_job_group_status(ui, &api_client, group_id.unwrap(), show_jobs)?;
    }

    Ok(())
}

fn do_job_group_status(ui: &mut UI,
                       api_client: &api_client::BoxedClient,
                       group_id: &str,
                       show_jobs: bool)
                       -> Result<()> {
    let gid = match group_id.parse::<i64>() {
        Ok(g) => g,
        Err(e) => {
            ui.fatal(format!("Failed to parse group id: {}", e))?;
            return Err(Error::ParseIntError(e));
        }
    };

    ui.status(Status::Determining,
              format!("status of job group {}", group_id))?;

    match api_client.get_schedule(gid, show_jobs) {
        Ok(sr) => {
            let mut tw = TabWriter::new(vec![]);
            writeln!(&mut tw, "CREATED AT\tGROUP ID\tSTATUS\tIDENT\tTARGET").unwrap();
            writeln!(&mut tw,
                     "{}\t{}\t{}\t{}\t{}",
                     sr.created_at, sr.id, sr.state, sr.project_name, sr.target).unwrap();
            tw.flush().unwrap();
            let mut written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
            println!("\n{}", written);

            if show_jobs && !sr.projects.is_empty() {
                tw = TabWriter::new(vec![]);
                writeln!(&mut tw, "NAME\tSTATUS\tJOB ID\tIDENT\tTARGET").unwrap();
                for p in sr.projects {
                    // Don't show ident if the build did not succeed
                    // TODO: This will be fixed at the API level eventually
                    let ident = if p.state == "Success" { &p.ident } else { "" };
                    writeln!(&mut tw,
                             "{}\t{}\t{}\t{}\t{}",
                             p.name, p.state, p.job_id, ident, p.target).unwrap();
                }
                written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
                println!("{}", written);
            }
            Ok(())
        }
        Err(e) => Err(Error::ScheduleStatus(e)),
    }
}

fn do_origin_status(ui: &mut UI,
                    api_client: &api_client::BoxedClient,
                    origin: &str,
                    limit: usize)
                    -> Result<()> {
    ui.status(Status::Determining,
              format!("status of job groups in {} origin", origin))?;

    match api_client.get_origin_schedule(origin, limit) {
        Ok(sr) => {
            let mut tw = TabWriter::new(vec![]);
            writeln!(&mut tw, "CREATED AT\tGROUP ID\tSTATUS\tIDENT\tTARGET").unwrap();
            for s in sr.iter() {
                writeln!(&mut tw,
                         "{}\t{}\t{}\t{}\t{}",
                         s.created_at, s.id, s.state, s.project_name, s.target,).unwrap();
            }

            tw.flush().unwrap();
            let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
            println!("\n{}", written);
            Ok(())
        }
        Err(e) => Err(Error::ScheduleStatus(e)),
    }
}
