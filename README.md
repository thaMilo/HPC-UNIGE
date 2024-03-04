<a href="https://typst.app/project/ryNYsH-JQNzMXMaxWrrel5">
<img src="./Banner.svg" />
</a>

# Installation

## Build from source

```bash
git clone
cd Mandelbrot/mandelbrot_lab
make all
```

## Usage

### Running specific optimization profile

```bash
cd Mandelbrot/mandelbrot_lab
cargo run --profile <profile>
```

> Replace `<profile>` with the desired profile name.
> Possible profiles are: `opt-0`, `opt-1`, `opt-2`, `opt-3`, `opt-native`.