
extern crate tempfile;

use std::env;
use std::process;
use std::str;

static SOURCE_PLAIN: &str = "
    fn main() {
        ::std::process::exit(0);
    }
";

static SOURCE_ENCAPSULATE: &str = "
    #[macro_use] extern crate encapsulate;
    fn main() {
        encapsulate! { ::std::process::exit(0) };
    }
";

macro_rules! gen_opt_level_tests {
    ($name:ident, $source_a:expr, $source_b:expr, $opt_level:expr) => {

        #[test]
        fn $name() {
            let lib = ::generate_encapsulate();
            ::assert_eq_asm(&lib, $source_a, $source_b, $opt_level);
        }
    }
}

macro_rules! gen_tests {
    ($name:ident, $source_a:expr, $source_b:expr) => {
        mod $name {
            gen_opt_level_tests!(opt_level_0, $source_a, $source_b, 0);
            gen_opt_level_tests!(opt_level_1, $source_a, $source_b, 1);
            gen_opt_level_tests!(opt_level_2, $source_a, $source_b, 2);
            gen_opt_level_tests!(opt_level_3, $source_a, $source_b, 3);
        }
    }
}

gen_tests!(encapsulate_vs_plain, ::SOURCE_ENCAPSULATE, ::SOURCE_PLAIN);

fn assert_eq_asm(
    lib: &tempfile::NamedTempFile,
    source_a: &str,
    source_b: &str,
    opt_level: u8,
) {
    let asm_a = generate_asm(&lib, source_a, opt_level);
    let asm_b = generate_asm(&lib, source_b, opt_level);
    if asm_a != asm_b {
        panic!("ASM mismatch:\n\n## ASM A:\n{}\n\n## ASM B:\n{}\n", asm_a, asm_b);
    }
}

fn generate_encapsulate() -> tempfile::NamedTempFile {

    let rustc = env::var("ENCAPSULATE_RUSTC")
        .expect("discovered rustc binary");

    let lib_file = tempfile::Builder::new()
        .prefix("lib")
        .suffix(".rlib")
        .tempfile()
        .expect("temporary lib file");

    let rustc_output = process::Command::new(rustc)
        .arg("-o")
        .arg(lib_file.path())
        .arg("--crate-name")
        .arg("encapsulate")
        .arg("--crate-type")
        .arg("rlib")
        .arg(&format!("{}/src/lib.rs", env!("CARGO_MANIFEST_DIR")))
        .output()
        .expect("successful rustc invocation");

    if !rustc_output.status.success() {
        let stderr = str::from_utf8(&rustc_output.stderr)
            .expect("rustc outputs utf-8 to stderr");
        panic!("rustc failed:\n{}", stderr);
    }

    lib_file
}

fn generate_asm(
    lib: &tempfile::NamedTempFile,
    input: &str,
    opt_level: u8,
) -> String {
    use std::io::{Read, Write};

    let rustc = env::var("ENCAPSULATE_RUSTC")
        .expect("discovered rustc binary");

    let mut source_file = tempfile::NamedTempFile::new()
        .expect("temporary source file");
    write!(source_file, "{}", input)
        .expect("ability to write source file");

    let mut asm_file = tempfile::NamedTempFile::new()
        .expect("temporary asm file");

    let rustc_output = process::Command::new(rustc)
        .arg("-o")
        .arg(asm_file.path())
        .arg("--crate-name")
        .arg("encapsulate_asm_test")
        .arg("--emit")
        .arg("asm")
        .arg("-C")
        .arg(&format!("opt-level={}", opt_level))
        .arg("--extern")
        .arg(&format!("encapsulate={}", lib.path().to_str().unwrap()))
        .arg(source_file.path())
        .output()
        .expect("successful rustc invocation");

    if !rustc_output.status.success() {
        let stderr = str::from_utf8(&rustc_output.stderr)
            .expect("rustc outputs utf-8 to stderr");
        panic!("rustc failed:\n{}", stderr);
    }

    let mut asm = String::new();
    asm_file.read_to_string(&mut asm)
        .expect("reading asm file");

    asm
}
