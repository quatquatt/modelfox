use tangram_charts::{bar_chart::BarChart, components::hydrate_chart, line_chart::LineChart};
use wasm_bindgen::{self, prelude::*};

#[wasm_bindgen(start)]
pub fn start() {
	console_error_panic_hook::set_once();
	hydrate_chart::<LineChart>("loss");
	hydrate_chart::<BarChart>("feature_importances");
}