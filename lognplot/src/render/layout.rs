use super::ChartOptions;
use crate::geometry::Size;

/// Chart layout in pixels.
///
/// This struct has the various elements where parts of the chart are located.
pub struct ChartLayout {
    pub width: f64,
    pub height: f64,
    pub y_axis_legend_width: f64,
    pub title_height: f64,
    pub x_axis_legend_height: f64,
    pub info_bar_height: f64,
    pub plot_top: f64,
    pub plot_left: f64,
    pub plot_bottom: f64,
    pub plot_right: f64,
    pub plot_width: f64,
    pub plot_height: f64,
}

impl ChartLayout {
    pub fn new(size: Size) -> Self {
        ChartLayout {
            // TODO: casowary?
            width: size.width,
            y_axis_legend_width: 140.0,
            x_axis_legend_height: 60.0,
            title_height: 0.0,
            info_bar_height: 10.0,
            height: size.height,
            plot_top: 0.0,
            plot_left: 0.0,
            plot_bottom: 0.0,
            plot_right: 0.0,
            plot_width: 0.0,
            plot_height: 0.0,
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn layout(&mut self, options: &ChartOptions) {
        self.plot_top = options.padding + self.title_height;
        self.plot_left = self.y_axis_legend_width;
        self.plot_bottom = self.height
            - (self.x_axis_legend_height + options.padding * 2.0 + self.info_bar_height);
        self.plot_right = self.width - options.padding;
        self.plot_height = self.plot_bottom - self.plot_top;
        self.plot_width = self.plot_right - self.plot_left;
    }
}
