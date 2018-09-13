#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate structopt;

pub extern crate failure;
pub extern crate glium;
pub extern crate glium_sdl2;
pub extern crate image;
pub extern crate nalgebra;
pub extern crate sdl2;

pub mod helpers;
#[macro_use]
mod macros;

use std::process::exit;

use failure::{err_msg, Error};
use glium_sdl2::{DisplayBuild, SDL2Facade};
use sdl2::{event::Event, video::GLProfile, Sdl};
use structopt::{
    clap::{App, Arg, ArgMatches},
    StructOpt,
};

/// A wrapper for a graphics loop, including SDL and OpenGL initialization and event loop setup.
///
/// Also handles the SQL Quit event.
pub fn run_wrapper<FE, FI, FL, T, U>(
    name: &str,
    init_func: FI,
    mut loop_func: FL,
    mut event_func: FE,
) where
    FE: FnMut(Event, &mut U, &mut Sdl, &mut SDL2Facade) -> Result<(), Error>,
    FI: FnOnce(T, &mut Sdl, &mut SDL2Facade) -> Result<U, Error>,
    FL: FnMut(&mut U, &mut Sdl, &mut SDL2Facade) -> Result<bool, Error>,
    T: StructOpt,
{
    let options = Args::from_args();
    if !options.quiet {
        let r = ::stderrlog::new().verbosity(options.verbose).init();
        if let Err(err) = r {
            error!("Warning: logging couldn't start: {}", err);
        }
    }

    if options.verbose == 0 {
        setup_panic();
    }

    let run = || -> Result<(), Error> {
        let mut sdl = sdl2::init().map_err(err_msg)?;
        let video_subsystem = sdl.video().map_err(err_msg)?;
        let gl_attrs = video_subsystem.gl_attr();
        gl_attrs.set_context_major_version(3);
        gl_attrs.set_context_minor_version(2);
        gl_attrs.set_context_profile(GLProfile::Core);
        let mut display = video_subsystem
            .window(name, 800, 600)
            .resizable()
            .build_glium()?;
        let mut events = sdl.event_pump().map_err(err_msg)?;
        let mut state = init_func(options.t, &mut sdl, &mut display)?;
        'main_loop: loop {
            if !loop_func(&mut state, &mut sdl, &mut display)? {
                break Ok(());
            }
            for event in events.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'main_loop Ok(());
                } else {
                    event_func(event, &mut state, &mut sdl, &mut display)?;
                }
            }
        }
    };

    if let Err(err) = run() {
        let mut first = true;
        let num_errs = err.iter_chain().count();
        if num_errs <= 1 {
            error!("{}", err);
        } else {
            for cause in err.iter_chain() {
                if first {
                    first = false;
                    error!("           {}", cause);
                } else {
                    error!("caused by: {}", cause);
                }
            }
        }
        debug!("{}", err.backtrace());
        exit(1);
    }
}

#[cfg(debug_assertions)]
fn setup_panic() {}

#[cfg(not(debug_assertions))]
fn setup_panic() {
    setup_panic!();
}

struct Args<T> {
    /// Turns off message output.
    quiet: bool,

    /// Increases the verbosity. Default verbosity is errors only.
    verbose: usize,

    t: T,
}

#[allow(unused_variables)]
impl<T: StructOpt> StructOpt for Args<T> {
    fn clap<'a, 'b>() -> App<'a, 'b> {
        let app = App::new("common")
            .version("0.1.0")
            .author("Nathan Ringo <remexre@gmail.com>");
        <T>::clap()
            .arg(
                Arg::with_name("quiet")
                    .takes_value(false)
                    .multiple(false)
                    .help("Turns off message output.")
                    .short("q")
                    .long("quiet"),
            )
            .arg(
                Arg::with_name("verbose")
                    .takes_value(false)
                    .multiple(true)
                    .help("Increases the verbosity. Default verbosity is errors only.")
                    .short("v")
                    .long("verbose"),
            )
    }

    fn from_clap(matches: &ArgMatches) -> Self {
        Args {
            quiet: matches.is_present("quiet"),
            verbose: { |v| v as _ }(matches.occurrences_of("verbose")),
            t: StructOpt::from_clap(matches),
        }
    }
}
