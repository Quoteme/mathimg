use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} --packages=\"pak1,pak2\" \"equation text\"",
            args[0]
        );
        std::process::exit(1);
    }

    // Parse optional parameters
    let packages_arg = args.iter().find(|arg| arg.starts_with("--packages="));
    let packages = packages_arg.map(|arg| {
        arg.strip_prefix("--packages=")
            .unwrap()
            .split(',')
            .collect::<Vec<_>>()
    });

    // Get the equation
    let equation = args.last().unwrap();

    // Define LaTeX content
    let mut latex_content = String::from("\\documentclass{standalone}\n");
    if let Some(pkgs) = packages {
        for pkg in pkgs {
            latex_content.push_str(&format!("\\usepackage{{{}}}\n", pkg));
        }
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
    let output = Command::new("pdflatex")
        .args(&["-output-directory=/tmp", latex_file_path])
        .output()
        .expect("Failed to execute pdflatex");

    if !output.status.success() {
        eprintln!("pdflatex error: {:?}", output);
        std::process::exit(1);
    }

    // Convert PDF to SVG using dvisvgm
    let pdf_file_path = "/tmp/equation.pdf";
    let svg_file_path = "/tmp/equation.svg";
    let output = Command::new("dvisvgm")
        .args(&[pdf_file_path, "-o", svg_file_path])
        .output()
        .expect("Failed to execute dvisvgm");

    if !output.status.success() {
        eprintln!("dvisvgm error: {:?}", output);
        std::process::exit(1);
    }

    println!("SVG file created at: {}", svg_file_path);
}
