use pinwheel::prelude::*;
use tangram_serve::hash;

#[derive(ComponentBuilder)]
pub struct Document {
	#[optional]
	pub client: Option<&'static str>,
	#[children]
	pub children: Vec<Node>,
}

impl Component for Document {
	fn into_node(self) -> Node {
		let head = head()
			.child(meta().attribute("charset", "utf-8"))
			.child(
				meta()
					.attribute("content", "width=device-width, initial-scale=1")
					.attribute("name", "viewport"),
			)
			.child(
				link()
					.attribute("href", "/favicon.png")
					.attribute("rel", "icon")
					.attribute("type", "image/png"),
			)
			.child(title().child("Tangram"))
			.child(
				link()
					.attribute("href", "/styles.css")
					.attribute("rel", "stylesheet"),
			)
			.child(
				meta()
					.attribute(
						"content",
						"All-In-One Machine Learning Toolkit Designed for Programmers",
					)
					.attribute("name", "description"),
			);
		let client_script = self.client.map(|client| {
			script().attribute("type", "module").inner_html(format!(
				r#"import init from "/js/{hash}.js"; init("/js/{hash}_bg.wasm")"#,
				hash = hash(client),
			))
		});
		let body = body().child(self.children).child(client_script);
		html::html().child(head).child(body).into_node()
	}
}