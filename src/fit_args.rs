use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

// Program to detect elongated particles on images
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Pathname of the image to open
    #[clap(short, long, value_parser)]
    pub file: PathBuf,

    /// Pathname of the config file for filtering
    #[clap(short, long, value_parser)]
    pub config: PathBuf,

    /// Pathname for the output JSON file
    /// If not specified, the output file is omitted
    #[clap(long, value_parser)]
    pub outfile: Option<PathBuf>,

    /// Pathname for the output image file
    /// If not specified, the output image is omitted
    #[clap(long, value_parser)]
    pub outimg: Option<PathBuf>,

    /// Verbosity level
    #[clap(short, long, parse(from_occurrences))]
    pub verbosity: usize,

    /// Multiplier for the number of random samples to take.
    /// If not specified, the default is 10. The higher the number,
    /// the more accurate the fit will be, but also the computation takes longer.
    #[clap(long, value_parser, default_value = "10")]
    pub samplemult: f64,

    /// If used, the program will utilize all available cores, otherwise it will use only one core.
    #[clap(long, parse(from_occurrences))]
    pub multithread: usize,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct FitArgs {
    /// Threshold for binarization
    pub threshold: u8,

    /// Minimum fitness value for ellipse fitting
    pub min_fitness: f64,

    /// Minimum distance for contour points to count towards fitness (in pixel units)
    pub dist_threshold: f64,

    /// Radius threshold for random sampling (in pixel units)
    pub radius_threshold: f64,

    /// Minimum number of points in a contour
    pub min_contour_points: usize,

    /// Maximum number of points in a contour
    pub max_contour_points: usize,

    /// Minimum value of particle aspect ratio (must be >= 1.0, since the aspect ratio is always calculated to be >= 1.0)
    pub min_aspect_ratio: f64,

    /// Maximum value of particle aspect ratio (must be >= 1.0, since the aspect ratio is always calculated to be >= 1.0)
    pub max_aspect_ratio: f64,

    /// Minimum length of a particle, in pixels
    pub min_length: f64,

    /// Maximum length of a particle, in pixels
    pub max_length: f64,

    /// Minimum width of a particle, in pixels
    pub min_width: f64,

    /// Maximum width of a particle, in pixels
    pub max_width: f64,

    /// Center of rotation, x coordinate
    pub rotation_center_x: f64,

    /// Center of rotation, y coordinate
    pub rotation_center_y: f64,

    /// Minimum detection radius measured from the center of rotation
    pub detect_radius_min: f64,

    /// Maximum detection radius measured from the center of rotation
    pub detect_radius_max: f64,
}
