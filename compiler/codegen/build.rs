extern crate cmake;
extern crate walkdir;
extern crate which;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::{DirEntry, WalkDir};

const ENV_LLVM_PREFIX: &'static str = "LLVM_PREFIX";
const ENV_LLVM_BUILD_STATIC: &'static str = "LLVM_BUILD_STATIC";

fn main() {
    // Emit custom cfg types:
    //     cargo:rustc-cfg=has_foo
    // Can then be used as `#[cfg(has_foo)]` when emitted

    // Emit custom env data:
    //     cargo:rustc-env=foo=bar
    // Can then be fetched with `env!("foo")`

    // LLVM
    let target = env::var("TARGET").unwrap();

    let cwd = env::current_dir().expect("unable to access current directory");
    let codegen_llvm = cwd.join("../codegen_llvm");
    let llvm_prefix = detect_llvm_prefix();

    println!("cargo:rerun-if-env-changed={}", ENV_LLVM_PREFIX);
    println!("cargo:rerun-if-env-changed={}", ENV_LLVM_BUILD_STATIC);

    if let Err(_) = which::which("cmake") {
        fail(
            "Unable to locate CMake!\n\
             It is required for the build, make sure you have a recent version installed.",
        );
    }

    let mut use_ninja = true;
    if let Err(_) = which::which("ninja") {
        use_ninja = false;
        warn(
            "Unable to locate Ninja, your CMake builds may take unncessarily long.\n\
             It is highly recommended that you install Ninja.",
        );
    }

    let mut config = &mut cmake::Config::new(&codegen_llvm);
    if use_ninja {
        config = config.generator("Ninja");
    }
    let build_shared = if env::var_os(ENV_LLVM_BUILD_STATIC).is_some() {
        "OFF"
    } else {
        "ON"
    };

    let lumen_llvm_include_dir = env::var("DEP_LUMEN_LLVM_CORE_INCLUDE").unwrap();
    let lumen_mlir_include_dir = env::var("DEP_LUMEN_MLIR_CORE_INCLUDE").unwrap();
    let lumen_term_include_dir = env::var("DEP_LUMEN_TERM_CORE_INCLUDE").unwrap();

    rerun_if_changed_anything_in_dir(&codegen_llvm);

    let outdir = config
        .define("LUMEN_BUILD_COMPILER", "ON")
        .define("LUMEN_BUILD_TESTS", "OFF")
        .define("LLVM_BUILD_LLVM_DYLIB", build_shared)
        .define("LLVM_LINK_LLVM_DYLIB", build_shared)
        .define("LLVM_PREFIX", llvm_prefix.as_path())
        .env("LLVM_PREFIX", llvm_prefix.as_path())
        .cxxflag(&format!("-I{}", lumen_llvm_include_dir))
        .cxxflag(&format!("-I{}", lumen_mlir_include_dir))
        .cxxflag(&format!("-I{}", lumen_term_include_dir))
        .always_configure(true)
        .build_target("install")
        .very_verbose(false)
        .build();

    let lumen_term_output_dir = env::var("DEP_LUMEN_TERM_CORE_OUTPUT_DIR").unwrap();
    println!(
        "cargo:rustc-env=TERM_LIB_OUTPUT_DIR={}",
        lumen_term_output_dir
    );

    let compile_commands_src = outdir.join("build").join("compile_commands.json");
    let compile_commands_dest = codegen_llvm.join("lib").join("compile_commands.json");

    fs::copy(compile_commands_src, compile_commands_dest)
        .expect("unable to copy compile_commands.json!");

    println!("cargo:rustc-link-search=native={}/lib", outdir.display());

    link_libs(&["lumen_EIR_IR", "lumen_EIR_Conversion", "lumen_EIR_Builder"]);

    // Get demangled lang_start_internal name

    let mut sysroot_cmd = Command::new("rustc");
    let mut sysroot_cmd = sysroot_cmd.args(&["--print", "sysroot"]);
    let sysroot = PathBuf::from(output(&mut sysroot_cmd).trim());
    let toolchain_libs = sysroot.join("lib/rustlib").join(target).join("lib");
    let libstd_rlib = toolchain_libs
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| {
            let path = e.path();
            let filename = path.file_name().map(|s| s.to_string_lossy());
            if let Some(fname) = filename {
                if fname.starts_with("libstd") && fname.ends_with(".rlib") {
                    return true;
                }
            }
            false
        })
        .take(1)
        .next()
        .map(|e| e.path().to_string_lossy().into_owned())
        .expect("unable to find libstd rlib in toolchain directory!");

    let llvm_objdump = llvm_prefix.join("bin/llvm-objdump");
    let mut objdump_cmd = Command::new(llvm_objdump);
    let objdump_cmd = objdump_cmd
        .args(&["--syms", &libstd_rlib])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to open llvm-objdump");
    let mut grep_cmd = Command::new("grep");
    let grep_cmd = grep_cmd
        .args(&["lang_start_internal"])
        .stdin(objdump_cmd.stdout.unwrap())
        .stderr(Stdio::inherit());

    let results = output(grep_cmd);
    let lang_start_symbol = results
        .trim()
        .split(' ')
        .last()
        .expect("expected non-empty lang_start_symbol result");

    println!(
        "cargo:rustc-env=LANG_START_SYMBOL_NAME={}",
        lang_start_symbol_name(lang_start_symbol)
    );
}

pub fn output(cmd: &mut Command) -> String {
    let output = match cmd.stderr(Stdio::inherit()).output() {
        Ok(status) => status,
        Err(e) => fail(&format!(
            "failed to execute command: {:?}\nerror: {}",
            cmd, e
        )),
    };
    if !output.status.success() {
        panic!(
            "command did not execute successfully: {:?}\n\
             expected success, got: {}",
            cmd, output.status
        );
    }
    String::from_utf8(output.stdout).unwrap()
}

fn rerun_if_changed_anything_in_dir(dir: &Path) {
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !ignore_changes(e)) {
        let entry = entry.unwrap();
        if !entry.file_type().is_dir() {
            let path = entry.path();
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn ignore_changes(entry: &DirEntry) -> bool {
    let ty = entry.file_type();
    let name = entry.file_name().to_string_lossy();
    // Ignore hidden files and directories
    if name.starts_with(".") {
        return true;
    }
    // Recurse into subdirectories
    if ty.is_dir() {
        return false;
    }
    // CMake build changes, or any C++/TableGen changes should not be ignored
    if name == "CMakeLists.txt" {
        return false;
    }
    if name.ends_with(".cpp") || name.ends_with(".h") || name.ends_with(".td") {
        return false;
    }
    // Ignore everything else
    true
}

#[cfg(target_os = "macos")]
fn lang_start_symbol_name(lang_start_symbol: &str) -> &str {
    // Strip off leading `_` when printing symbol name
    &lang_start_symbol[1..]
}

#[cfg(not(target_os = "macos"))]
fn lang_start_symbol_name(lang_start_symbol: &str) -> &str {
    lang_start_symbol
}

fn link_libs(libs: &[&str]) {
    if env::var_os(ENV_LLVM_BUILD_STATIC).is_none() {
        link_libs_dylib(libs);
    } else {
        link_libs_static(libs);
    }
}

#[inline]
fn link_libs_static(libs: &[&str]) {
    for lib in libs {
        link_lib_static(lib);
    }
}

#[inline]
fn link_libs_dylib(libs: &[&str]) {
    for lib in libs {
        link_lib_dylib(lib);
    }
}

#[inline]
fn link_lib_static(lib: &str) {
    println!("cargo:rustc-link-lib=static={}", lib);
}

#[inline]
fn link_lib_dylib(lib: &str) {
    println!("cargo:rustc-link-lib=dylib={}", lib);
}

fn warn(s: &str) {
    println!("cargo:warning={}", s);
}

fn detect_llvm_prefix() -> PathBuf {
    if let Ok(prefix) = env::var(ENV_LLVM_PREFIX) {
        return PathBuf::from(prefix);
    }

    if let Ok(llvm_config) = which::which("llvm-config") {
        let mut cmd = Command::new(llvm_config);
        cmd.arg("--prefix");
        return PathBuf::from(output(&mut cmd));
    }

    let mut llvm_prefix = env::var("XDG_DATA_HOME")
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|_| {
            let mut home = PathBuf::from(env::var("HOME").expect("HOME not defined"));
            home.push(".local/share");
            home
        });
    llvm_prefix.push("llvm");
    if llvm_prefix.exists() {
        // Make sure its actually the prefix and not a root
        let llvm_bin = llvm_prefix.as_path().join("bin");
        if llvm_bin.exists() {
            return llvm_prefix;
        }
        let lumen = llvm_prefix.as_path().join("lumen");
        if lumen.exists() {
            return lumen.to_path_buf();
        }
    }

    fail("LLVM_PREFIX is not defined and unable to locate LLVM to build with");
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
