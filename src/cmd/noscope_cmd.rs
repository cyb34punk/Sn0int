use clap::Parser;
use crate::cmd::Cmd;
use crate::errors::*;
use crate::filters::{Target, Filter};
use crate::models::*;
use crate::shell::Shell;
use crate::term;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    subcommand: Target,
}

impl Cmd for Args {
    fn run(self, rl: &mut Shell) -> Result<()> {
        let rows = match self.subcommand {
            Target::Domains(filter) => noscope::<Domain>(rl, &filter),
            Target::Subdomains(filter) => noscope::<Subdomain>(rl, &filter),
            Target::IpAddrs(filter) => noscope::<IpAddr>(rl, &filter),
            Target::Urls(filter) => noscope::<Url>(rl, &filter),
            Target::Emails(filter) => noscope::<Email>(rl, &filter),
            Target::PhoneNumbers(filter) => noscope::<PhoneNumber>(rl, &filter),
            Target::Devices(filter) => noscope::<Device>(rl, &filter),
            Target::Networks(filter) => noscope::<Network>(rl, &filter),
            Target::Accounts(filter) => noscope::<Account>(rl, &filter),
            Target::Breaches(filter) => noscope::<Breach>(rl, &filter),
            Target::Images(filter) => noscope::<Image>(rl, &filter),
            Target::Ports(filter) => noscope::<Port>(rl, &filter),
            Target::Netblocks(filter) => noscope::<Netblock>(rl, &filter),
            Target::CryptoAddrs(filter) => noscope::<CryptoAddr>(rl, &filter),
        }?;
        term::info(&format!("Updated {} rows", rows));
        Ok(())
    }
}

pub fn run(rl: &mut Shell, args: &[String]) -> Result<()> {
    let args = Args::try_parse_from(args)?;
    args.run(rl)
}

#[inline]
fn noscope<T: Model + Detailed>(rl: &mut Shell, filter: &Filter) -> Result<usize> {
    T::noscope(rl.db(), &filter.parse()?)
}
