extern crate clap;

use self::clap::{App, Arg};

#[derive(Debug)]
pub struct Options {
    pub size: (u32, u32),
    pub scale: u32,
}

pub fn parse_cl() -> Options {
    let matches = App::new(env!("CARGO_PKG_NAME"))
              .version(env!("CARGO_PKG_VERSION"))
              .about(env!("CARGO_PKG_DESCRIPTION"))
              .author(env!("CARGO_PKG_AUTHORS"))
              .arg(Arg::with_name("width")
                    .short("x")
                    .long("width")
                    .help("the number of tiles in the horizontal direction")
                    .takes_value(true)
                    .conflicts_with("square")
              )
              .arg(Arg::with_name("height")
                    .short("y")
                    .long("height")
                    .help("the number of tiles in the vertical direction")
                    .takes_value(true)
                    .conflicts_with("square")
              )
              .arg(Arg::with_name("square")
                    .long("square")
                    .help("the number of tiles in both directions")
                    .takes_value(true)
                    .conflicts_with("width")
                    .conflicts_with("height")
              )
              .arg(Arg::with_name("scale")
                    .short("s")
                    .long("scale")
                    .default_value("20")
                    .help("size of each tile in pixel")
                    .takes_value(true)
              )
              .get_matches();

    let mut height = matches.value_of("height")
                            .and_then(|s| Some(s.parse::<u32>().expect("height needs to be an integer")));
    let mut width = matches.value_of("width")
                            .and_then(|s| Some(s.parse::<u32>().expect("width needs to be an integer")));
    let square = matches.value_of("square")
                        .and_then(|s| Some(s.parse::<u32>().expect("square needs to be an integer")));
    let scale = matches.value_of("scale")
                       .unwrap()
                       .parse::<u32>().expect("scale needs to be an integer");

    if let Some(s) = square {
        height = Some(s);
        width = Some(s);
    };

    let x = match width {
        Some(w) => w,
        None => 20
    };
    let y = match height {
        Some(w) => w,
        None => 20
    };

    Options {
        size: (x, y),
        scale
    }
}
