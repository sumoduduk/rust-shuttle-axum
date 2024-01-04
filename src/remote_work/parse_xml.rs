use crate::utils::parse_date;

use super::{mapped_detail::mapped_detail, JobPost};
use rss::Channel;
use std::io::BufRead;

pub fn parse_xml<R>(reader: R) -> eyre::Result<Vec<JobPost>>
where
    R: BufRead,
{
    let channel = Channel::read_from(reader)?;

    let items = channel.items;
    let len = items.len();

    let mut data: Vec<JobPost> = Vec::with_capacity(len);

    for item in items {
        let desc = item.description;

        match desc {
            Some(description) => {
                let title = item.title.unwrap_or_default();
                let link = item.link.unwrap_or_default();

                let posted_on = item.pub_date.unwrap_or_default();

                let timestamp = parse_date(&posted_on)?;

                let job_post = mapped_detail(posted_on, timestamp, title, link, description)?;

                data.push(job_post);
            }
            None => continue,
        }
    }

    Ok(data)
}
