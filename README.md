# Robust ellipse fitting written in Rust

This program implements the RANSAC based robust ellipse fitting algorithm described by [W Kaewapichai, P Kaewtrakulpong (2008)](https://scholar.google.com/scholar?cluster=5586026904313573649&hl=en&as_sdt=2007). This implementation is written purely in Rust. It is capable of detecting ellipse shaped blobs where the ellipse is partially occluded or more of them are clustered together into a single blob.

![Example picture](example.png)

## Example usage

Building is simple using `cargo`:

```shell
cargo build --release
```

For a short description of the available command line options, run:

```shell
./target/release/particle_detect -h
```

In the `example_use` directory, an example configuration file is provided `config.yaml`, along with a sample image `img.bmp`. We can run the program with the following command:

```shell
./target/release/particle_detect -c example_use/config.yaml -f example_use/img.bmp --outimg example_use/out.png --outfile example_use/out.json
```

Which results in a JSON file `out.json` and an image `out.png` containing and showing the detected ellipses. Both output files are optional.

The accepted input and output image formats are `png`, `bmp` and `jpg`.
