/*
MIT License

Copyright (c) 2016-2019 Janne Blomqvist

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

 */

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use hostlist;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("expand")
                .short("e")
                .long("expand")
                .help("Expand the hostlist given (default)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("contract")
                .short("c")
                .long("contract")
                .help("Contract the list of hosts given")
                .takes_value(false)
                .conflicts_with("expand"),
        )
        .arg(
            Arg::with_name("HOSTLIST")
                .help("The hostlist or list of hosts to expand/contract")
                .required(true)
                .index(1),
        )
        .get_matches();

    if matches.is_present("contract") {
        eprintln!("--contract is unimplemented, sorry!");
        return;
    }

    let hl = matches.value_of("HOSTLIST").unwrap();
    let hl = hostlist::expand(hl);
    match hl {
        Ok(o) => {
            let mut i = true;
            for e in o {
                if i {
                    print!("{}", e);
                    i = false;
                } else {
                    print!(",{}", e);
                }
            }
            println!("");
        }
        Err(e) => eprintln!("{}", e),
    }
}
