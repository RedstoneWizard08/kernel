use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CheckTarget {
    All,
    Kernel,
    Symbols,
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "==== The build system for the D.E.S.K. kernel. ====", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = false, help = "Enable verbose logging.")]
    verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Build the kernel.")]
    Build {
        #[arg(short, long, default_value_t = false, help = "Cleans the build output before building.")]
        clean: bool,

        #[arg(short, long, default_value_t = false, help = "Enable verbose logging.")]
        verbose: bool,
    },

    #[command(about = "Clean the build output.")]
    Clean {
        #[arg(short, long, default_value_t = false, help = "Enable verbose logging.")]
        verbose: bool,
    },

    #[command(about = "Checks/lints the code.")]
    Check {
        #[arg(short, long, value_enum, default_value_t = CheckTarget::All)]
        target: CheckTarget,

        #[arg(short, long, default_value_t = false, help = "Enable verbose logging.")]
        verbose: bool,
    },

    #[command(about = "Tests the kernel.")]
    Test {
        #[arg(short, long, default_value_t = false, help = "Cleans the build output before building.")]
        clean: bool,

        #[arg(short, long, default_value_t = false, help = "Runs all tests.")]
        all: bool,

        #[arg(short, long, default_value_t = false, help = "Runs only unit tests.")]
        unit: bool,

        #[arg(short, long, default_value_t = false, help = "Runs only boot tests.")]
        boot: bool,

        #[arg(short, long, default_value_t = false, help = "Runs only integration tests.")]
        integration: bool,

        #[arg(short, long, default_value_t = false, help = "Enable verbose logging.")]
        verbose: bool,
    },

    #[command(about = "Runs the kernel in QEMU.")]
    Run {
        #[arg(short, long, default_value_t = false, help = "Cleans the build output before building.")]
        clean: bool,

        #[arg(short, long, default_value_t = false, help = "Enable verbose logging.")]
        verbose: bool,
    },

    #[command(about = "Runs debug scripts on the kernel.")]
    Debug {
        #[arg(short, long, default_value_t = false, help = "Cleans the build output before building.")]
        clean: bool,

        #[arg(short, long, default_value_t = true, help = "Uses GDB for debugging.")]
        gdb: bool,

        #[arg(short, long, default_value_t = false, help = "Uses OpenOCD for debugging.")]
        openocd: bool,

        #[arg(short, long, default_value_t = false, help = "Enable verbose logging.")]
        verbose: bool,
    },
}
