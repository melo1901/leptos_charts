#![feature(iter_map_windows)]

pub mod bar;
pub mod color;
pub mod pie;

pub use bar::{BarChart, BarChartOptions};
pub use color::{CalculatedColor, ChartColor, Color, Gradient, Palette, CATPPUCCIN_COLORS};
pub use pie::{PieChart, PieChartOptions};
