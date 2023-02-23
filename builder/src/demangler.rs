use regex::{Captures, Regex};
use rustc_demangle::demangle;
use std::{
    borrow::Cow,
    fs::{remove_file, File},
    io::{self, stdin, stdout, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

// All of this is a modified version of rustfilt's main.rs file!
// Find the original code here: https://github.com/luser/rustfilt/blob/master/src/main.rs

lazy_static! {
    static ref MANGLED_NAME_PATTERN: Regex = Regex::new(r"_(ZN|R)[\$\._[:alnum:]]*").unwrap();
}

#[inline]
pub fn demangle_line(line: &str, include_hash: bool) -> Cow<str> {
    MANGLED_NAME_PATTERN.replace_all(line, |captures: &Captures| {
        let demangled = demangle(&captures[0]);

        if include_hash {
            demangled.to_string()
        } else {
            format!("{:#}", demangled)
        }
    })
}

pub fn demangle_stream<R: BufRead, W: Write>(
    input: &mut R,
    output: &mut W,
    include_hash: bool,
) -> io::Result<()> {
    let mut buf = String::new();

    while input.read_line(&mut buf)? > 0 {
        {
            let demangled_line = demangle_line(&buf, include_hash);

            if cfg!(debug_assertions) && buf.ends_with('\n') {
                let line_ending = if buf.ends_with("\r\n") { "\r\n" } else { "\n" };
                debug_assert!(
                    demangled_line.ends_with(line_ending),
                    "Demangled line has incorrect line ending"
                );
            }

            output.write_all(demangled_line.as_bytes())?;
        }

        buf.clear();
    }

    Ok(())
}

pub enum InputType {
    Stdin,
    File(PathBuf),
}

impl InputType {
    pub fn demangle(&self, output: OutputType, include_hash: bool) -> io::Result<()> {
        match *self {
            InputType::Stdin => {
                let stdin = stdin();
                let mut lock = stdin.lock();
                output.write_demangled(&mut lock, include_hash)
            }

            InputType::File(ref path) => {
                output.write_demangled(&mut BufReader::new(File::open(path)?), include_hash)
            }
        }
    }

    pub fn validate(file: String) -> Result<(), String> {
        file.parse::<InputType>().map(|_| ())
    }
}

impl FromStr for InputType {
    type Err = String;

    fn from_str(file: &str) -> Result<InputType, String> {
        if file == "-" {
            Ok(InputType::Stdin)
        } else {
            let path = Path::new(&file);

            if !path.is_file() {
                if !path.exists() {
                    Err(format!("{} doesn't exist", file))
                } else {
                    Err(format!("{} isn't a file", file))
                }
            } else {
                Ok(InputType::File(PathBuf::from(path)))
            }
        }
    }
}

pub enum OutputType {
    Stdout,
    File(PathBuf),
}

impl OutputType {
    #[inline]
    pub fn write_demangled<I: io::BufRead>(
        &self,
        input: &mut I,
        include_hash: bool,
    ) -> io::Result<()> {
        match *self {
            OutputType::Stdout => {
                let stdout = stdout();
                let mut lock = stdout.lock();
                demangle_stream(input, &mut lock, include_hash)
            }

            OutputType::File(ref path) => {
                let file = File::create(path)?;
                let mut buf = BufWriter::new(&file);
                demangle_stream(input, &mut buf, include_hash)
            }
        }
    }

    pub fn write_demangled_names<S: AsRef<str>>(
        &self,
        names: &[S],
        include_hash: bool,
    ) -> io::Result<()> {
        #[inline]
        fn demangle_names_to<S: AsRef<str>, O: io::Write>(
            names: &[S],
            output: &mut O,
            include_hash: bool,
        ) -> io::Result<()> {
            for name in names {
                let demangled = demangle(name.as_ref());
                if include_hash {
                    writeln!(output, "{}", demangled)?
                } else {
                    writeln!(output, "{:#}", demangled)?
                };
            }
            Ok(())
        }

        match *self {
            OutputType::Stdout => {
                let stdout = stdout();
                let mut lock = stdout.lock();
                demangle_names_to(names, &mut lock, include_hash)
            }

            OutputType::File(ref path) => {
                let file = File::create(path)?;
                let mut buf = BufWriter::new(&file);
                demangle_names_to(names, &mut buf, include_hash)
            }
        }
    }

    pub fn validate(file: String) -> Result<(), String> {
        file.parse::<OutputType>().map(|_| ())
    }
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(file: &str) -> Result<OutputType, String> {
        if file == "-" {
            Ok(OutputType::Stdout)
        } else {
            let path = Path::new(&file);
            if path.exists() {
                Err(format!("{} already exists", file))
            } else {
                Ok(OutputType::File(PathBuf::from(path)))
            }
        }
    }
}

pub fn do_demangle(file_path: String, out_file: String) {
    if Path::new(&out_file).exists() {
        remove_file(Path::new(&out_file)).unwrap();
    }

    let input = file_path.parse::<InputType>().unwrap();
    let output = out_file.parse::<OutputType>().unwrap();

    input.demangle(output, false).unwrap();
}
