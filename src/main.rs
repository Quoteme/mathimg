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
        println!("  --output=filename        Specify the output file path (default: /tmp/equation.svg or /tmp/equation.png).");
        println!("  --png                    Export as PNG instead of SVG.");
        println!("  --height=value           Specify the height of the output image (only valid for PNG).");
        println!("  --width=value            Specify the width of the output image (only valid for PNG).");
        println!("  --help                   Display this help message.");
        println!("\nExamples:");
        println!(
            "  {0} \"\\$\\sqrt{{5}}\\$\"                # Inline math mode",
            args[0]
        );
        println!(
            "  {0} \"\\$\\$\\frac{{a}}{{b}}\\$\\$\"           # Display math mode",
            args[0]
        );
        println!(
            "  {0} --png \"\\$\\sqrt{{5}}\\$\"           # Export as PNG",
            args[0]
        );
        println!("  {0} --png --output=\"output.png\" \"\\$\\sqrt{{5}}\\$\"   # Specify output file and format", args[0]);
        println!(
            "  {0} --png --height=500 \"\\$\\sqrt{{5}}\\$\"   # Specify output height for PNG",
            args[0]
        );
        println!(
            "  {0} --png --width=800 \"\\$\\sqrt{{5}}\\$\"   # Specify output width for PNG",
            args[0]
        );
        println!("  {0} --png --height=500 --width=800 \"\\$\\sqrt{{5}}\\$\"   # Specify both dimensions for PNG", args[0]);
        println!("  {0} --output=\"output.svg\" \"\\$\\sqrt{{5}}\\$\"   # Specify output file and format for SVG", args[0]);
        std::process::exit(0);
    }

    // Parse optional parameters
    let packages_arg = args.iter().find(|arg| arg.starts_with("--packages="));
    let packages: Vec<&str> = packages_arg.map_or_else(
        || vec!["amsmath"],
        |arg| {
            arg.strip_prefix("--packages=")
                .unwrap()
                .split(',')
                .collect()
        },
    );

    let output_file_arg = args.iter().find(|arg| arg.starts_with("--output="));
    let output_file = output_file_arg.map_or("/tmp/equation.svg".to_string(), |arg| {
        arg.strip_prefix("--output=").unwrap().to_string()
    });

    let as_png = args.contains(&"--png".to_string());
    let mut output_file_with_extension = output_file.clone();
    if as_png {
        if !output_file_with_extension.ends_with(".png") {
            output_file_with_extension = "/tmp/equation.png".to_string();
        }
    } else if !output_file_with_extension.ends_with(".svg") {
        output_file_with_extension = "/tmp/equation.svg".to_string();
    }

    let height_arg = args.iter().find(|arg| arg.starts_with("--height="));
    let height_value = height_arg
        .map(|arg| arg.strip_prefix("--height=").unwrap())
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);

    let width_arg = args.iter().find(|arg| arg.starts_with("--width="));
    let width_value = width_arg
        .map(|arg| arg.strip_prefix("--width=").unwrap())
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);

    // Get the equation
    let equation = args.last().unwrap();

    // Define LaTeX content
    let mut latex_content = String::from("\\documentclass{standalone}\n");
    for pkg in &packages {
        latex_content.push_str(&format!("\\usepackage{{{}}}\n", pkg));
    }
    latex_content.push_str("\\begin{document}\n");
    latex_content.push_str(equation);
    latex_content.push_str("\n\\end{document}\n");

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

    // Fetch the dimensions of the PDF
    let pdf_file_path = "/tmp/equation.pdf";
    let output = run_command("pdfinfo", &[pdf_file_path]);

    if !output.status.success() {
        eprintln!("pdfinfo error: {:?}", output);
        std::process::exit(1);
    }

    let pdf_info = String::from_utf8_lossy(&output.stdout);
    let mut original_width = 0.0;
    let mut original_height = 0.0;

    for line in pdf_info.lines() {
        if line.starts_with("Page size:") {
            if let Some((width, height)) = parse_pdf_dimensions(&line) {
                original_width = width;
                original_height = height;
            }
            break;
        }
    }

    if original_width == 0.0 || original_height == 0.0 {
        eprintln!("Unable to parse PDF dimensions");
        std::process::exit(1);
    }

    let aspect_ratio = original_width / original_height;

    let target_width: u32;
    let target_height: u32;

    if height_value > 0 && width_value > 0 {
        target_width = width_value;
        target_height = height_value;
    } else if height_value > 0 {
        target_height = height_value;
        target_width = (height_value as f64 * aspect_ratio) as u32;
    } else if width_value > 0 {
        target_width = width_value;
        target_height = (width_value as f64 / aspect_ratio) as u32;
    } else {
        target_width = original_width as u32;
        target_height = original_height as u32;
    }

    // Convert PDF to the desired format
    if as_png {
        let mut pdftoppm_args = vec![
            "-png".to_string(),
            "-singlefile".to_string(),
            "-scale-to-x".to_string(),
            target_width.to_string(),
            "-scale-to-y".to_string(),
            target_height.to_string(),
            pdf_file_path.to_string(),
            output_file_with_extension
                .trim_end_matches(".png")
                .to_string(),
        ];

        let pdftoppm_args_refs: Vec<&str> = pdftoppm_args.iter().map(|arg| arg.as_str()).collect();
        let output = run_command("pdftoppm", &pdftoppm_args_refs);

        if !output.status.success() {
            eprintln!("pdftoppm error: {:?}", output);
            std::process::exit(1);
        }

        // Rename the generated file to add .png extension
        std::fs::rename(
            format!(
                "{}.png",
                output_file_with_extension.trim_end_matches(".png")
            ),
            output_file_with_extension.clone(),
        )
        .expect("Unable to rename file");

        println!("PNG file created at: {}", output_file_with_extension);
    } else {
        // Convert PDF to SVG using dvisvgm
        let output_svg = output_file_with_extension.clone();
        let output = run_command(
            "dvisvgm",
            &[pdf_file_path, "--pdf", "-n", "-o", &output_svg],
        );

        if !output.status.success() {
            eprintln!("dvisvgm error: {:?}", output);
            std::process::exit(1);
        }

        println!("SVG file created at: {}", output_svg);
    }
}

/// Helper function to parse PDF dimensions from pdfinfo output
fn parse_pdf_dimensions(line: &str) -> Option<(f64, f64)> {
    let mut parts = line.split_whitespace();
    parts.next(); // Skip "Page"
    parts.next(); // Skip "size:"
    let width = parts.next()?.parse().ok()?;
    parts.next(); // Skip "x"
    let height = parts.next()?.parse().ok()?;
    Some((width, height))
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
