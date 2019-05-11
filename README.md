# Ray tracing

A simple raytracer in Rust based on Peter Shirley's
[ Ray Tracing in One Weekend](https://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html)


## Usage

``` shell
cargo run --release -- <options>
```

## Help

``` shell
USAGE:
    raytracer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>        Output file. If not specified, wrties to stdout.
    -s, --samples <samples>    Numner of samples per pixel [default: 100]
    -t, --threads <threads>    Number of threads to run [default: 1]
    -x, --x_res <x_res>        Width of trace in pixels [default: 200]
    -y, --y_res <y_res>        Height of trace in pixels [default: 100]
```


## Sample Output
Well, this should look similar to all of the other projects following the
same book, but here is an example:

![Raytracer output: a few dozen raytraced spheres in a scene.](./images/random_scene.png "Sample
output.")

(Output is currently a .ppm; this file was converted.)
