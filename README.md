# MathImg

**MathImg** is a command-line tool written in Rust that converts LaTeX equations into high-quality SVG or PNG images. This tool is particularly useful for including mathematical equations in web pages, documents, or presentations.

![Logo](https://i.imgur.com/JbvMteR.png)

## Features

- Converts LaTeX equations to SVG or PNG images.
- Supports custom LaTeX packages.
- Flexible output options.
- Simple and easy-to-use interface.

## Installation

First, ensure you have the following dependencies installed:

- A LaTeX distribution (e.g., TeX Live)
- `pdflatex` for compiling LaTeX to PDF
- `dvisvgm` for converting PDF to SVG
- `pdftoppm` for converting PDF to PNG (part of `poppler-utils`)

### Debian-based systems

```sh
sudo apt-get install texlive texlive-latex-extra dvisvgm poppler-utils
```

### Building from Source

Clone the repository and build the project using `cargo`:

```sh
git clone https://github.com/your-username/mathimg.git
cd mathimg
cargo build --release
```

The compiled binary will be located in `target/release`.

## Usage

```sh
./target/release/mathimg [OPTIONS] equation
```

### Options

- `--packages="pkg1,pkg2"`: Comma-separated list of LaTeX packages to include (default: `amsmath`).
  Example: `--packages="amsmath,amsfonts,amssymb"`
- `--output=filename`: Specify the output file path (default: `/tmp/equation.svg` or `/tmp/equation.png`).
- `--png`: Export as PNG instead of SVG.
- `--help`: Display this help message.

### Examples

Render a simple equation:

```sh
./target/release/mathimg "$\\sqrt{5}$"
```

Render a fraction in display math mode:

```sh
./target/release/mathimg "$$\\frac{a}{b}$$"
```

Render using a custom package and output as PNG:

```sh
./target/release/mathimg --packages="amsmath,amssymb" --png "$\\mathbf{E = mc^2}$"
```

Specify a custom output file:

```sh
./target/release/mathimg --output="/path/to/output.svg" "$\\sum_{i=1}^n i = \\frac{n(n+1)}{2}$"
./target/release/mathimg --png --output="/path/to/output.png" "$\\sum_{i=1}^n i = \\frac{n(n+1)}{2}$"
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or new features. Here’s how you can contribute:

1. Fork this repository.
2. Create a new branch from `main` (e.g., `feature/awesome-feature`).
3. Commit your changes.
4. Push your branch.
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

This project uses the following tools and libraries:

- [pdflatex](https://www.latex-project.org/get/)
- [dvisvgm](https://dvisvgm.de/)
- [poppler-utils](https://poppler.freedesktop.org/)

## Contact

Feel free to reach out via GitHub issues for any queries.

---

Created with :heart: by [Quoteme](https://github.com/quoteme).
