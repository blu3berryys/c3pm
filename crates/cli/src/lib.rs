use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use std::thread;
use util::model::{Generator, Language};
use util::parse_language;

lazy_static! {
    pub static ref AVAILABLE_THREADS: usize = {
        thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    };
}

#[derive(Parser, Debug)]
pub struct C3pmArgs {
    /// Generate a new project :3
    #[command(subcommand)]
    pub subcommands: NewSubcmd,
}

#[derive(Subcommand, Debug)]
pub enum NewSubcmd {
    /// Generates a CMake project in a new directory
    New {
        /// The name of the project to generate
        name: String,

        /// The generator to use
        #[arg(
            short,
            long,
            required = false,
            help = "The name of the generator to use (use this flag without an argument for a list of possible values)",
            hide_possible_values = true
        )]
        generator: Option<Generator>,

        /// The language to use for the project (format the value as `{language (c/cpp)}{standard (valid standard num for language)})
        ///
        /// for example, `cpppm new example -l cpp14` would create a new c3pm project using C++14 in the directory `example`
        #[arg(short, long, value_parser=parse_language, default_value = "cpp23")]
        language: Language,

        /// The name of the folder to generate the project in (defaults to the project name)
        #[arg(short, long)]
        folder: Option<String>,
    },
    /// Initializes a new CMake project in the current directory
    Init {
        /// The name of the project to initialize (defaults to the name of the current directory)
        name: Option<String>,

        /// The generator to use
        #[arg(
            short,
            long,
            required = false,
            help = "The name of the generator to use (use this flag without an argument for a list of possible values)",
            hide_possible_values = true
        )]
        generator: Option<Generator>,

        /// The language to use for the project (format the value as `{language (c/cpp)}{standard (valid standard num for language)})
        ///
        /// for example, `cpppm init example -l c99` would initialize a new c3pm project using the C99 standard
        #[arg(short, long, value_parser=parse_language, default_value = "cpp23", required = false)]
        language: Language,
    },
    /// Builds the c3pm project
    Build {
        /// The number of threads to use for building
        #[arg(short = 'j', long = "jobs", default_value_t = *AVAILABLE_THREADS)]
        jobs: usize,

        /// The build config to use (e.g. Debug, RelWithDebInfo, Release)
        #[arg(short = 'c', long = "config", default_value = "RelWithDebInfo")]
        config: String,

        /// The generator to build with
        #[arg(short = 'g', required = false)]
        generator: Option<Generator>,
    },
    /// Cleans the c3pm project
    Clean {},
    /// Reconfigures the project
    Reconfigure {
        /// (optional) The generator to reconfigure with
        generator: Option<Generator>,
    },
}

pub fn cli() -> Result<(), String> {
    let args = C3pmArgs::parse();

    match args.subcommands {
        NewSubcmd::New {
            name,
            generator,
            language,
            folder,
        } => util::create_new_project(name, generator, language, folder)?,
        NewSubcmd::Init {
            name,
            generator,
            language,
        } => Ok(util::init_project_subcommand(name, generator, language)?),
        NewSubcmd::Build {
            jobs,
            config,
            generator,
        } => util::build_project(&jobs, &config, generator),
        NewSubcmd::Clean {} => Ok(util::clean_project().expect("fuck")),
        NewSubcmd::Reconfigure { generator } => {
            util::reconfigure_project_subcommand(generator);

            Ok(())
        }
    }
}
