use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Output};

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();

    // Help message
    if args.len() < 2 || args.contains(&"--help".to_string()) {
        println!("Usage: {} [OPTIONS] equation\n", args[0]);
        println!("Options:");
        println!("  --packages=\"pkg1,pkg2\"  Comma-separated list of LaTeX packages to include (default: amsmath).");
        println!("  --output=filename.svg    Specify the output SVG file path (default: /tmp/equation.svg).");
        println!("  --help                   Display this help message.");
        std::process::exit(0);
    }

    // Parse optional parameters
    let packages_arg = args.iter().find(|arg| arg.starts_with("--packages="));
    let packages = packages_arg.map_or_else(
        || vec!["amsmath"],
        |arg| {
            arg.strip_prefix("--packages=")
                .unwrap()
                .split(',')
                .collect::<Vec<_>>()
        },
    );

    let output_file_arg = args.iter().find(|arg| arg.starts_with("--output="));
    let output_file = output_file_arg.map_or("/tmp/equation.svg", |arg| {
        arg.strip_prefix("--output=").unwrap()
    });

    // Get the equation
    let equation = args.last().unwrap();

    // Define LaTeX content
    let mut latex_content = String::from("\\documentclass{standalone}\n");
    for pkg in packages.iter() {
        latex_content.push_str(&format!("\\usepackage{{{}}}\n", pkg));
    }
    latex_content.push_str("\\begin{document}\n\\[\n");
    latex_content.push_str(equation);
    latex_content.push_str("\n\\]\n\\end{document}\n");

    // Write LaTeX content to a temporary file
    let latex_file_path = "/tmp/equation.tex";
    let mut file = File::create(latex_file_path).expect("Unable to create file");
    file.write_all(latex_content.as_bytes())
        .expect("Unable to write data");

    // Run pdflatex to generate a PDF file
    let output = run_command("pdflatex", &["-output-directory=/tmp", latex_file_path]);

    if !output.status.success() {
        eprintln!("pdflatex error: {:?}", output);
        print_log("/tmp/equation.log");
        std::process::exit(1);
    }

    // Convert PDF to SVG using dvisvgm
    let pdf_file_path = "/tmp/equation.pdf";
    let output_svg = output_file;
    let output = run_command("dvisvgm", &[pdf_file_path, "-o", output_svg]);

    if !output.status.success() {
        eprintln!("dvisvgm error: {:?}", output);
        std::process::exit(1);
    }

    println!("SVG file created at: {}", output_svg);
}

/// Helper function to run a command and capture its output
fn run_command(command: &str, args: &[&str]) -> Output {
    Command::new(command)
        .args(args)
        .output()
        .expect(&format!("Failed to execute {}", command))
}

/// Helper function to print the content of the log file
fn print_log(log_path: &str) {
    let mut log_file = File::open(log_path).expect("Unable to open log file");
    let mut log_content = String::new();
    log_file
        .read_to_string(&mut log_content)
        .expect("Unable to read log file");
    eprintln!("{}", log_content);
}
