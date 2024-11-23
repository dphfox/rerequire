use anyhow::{bail, Context, Result};
use std::{env, fs::read_dir, path::{Path, PathBuf}};

#[derive(Debug)]
struct NodeModule {
	source_on_disk: PathBuf,
	dir_on_disk: Option<PathBuf>
}

#[derive(Debug)]
struct Node {
	parent_on_disk: PathBuf,
	name: String,
	module: Option<NodeModule>,
	subnodes: Vec<Node>
}

const LUAU_SOURCE_EXTENSIONS: &[&str] = &[
	"lua",
	"luau"
];

fn discover_nodes(
	parent_on_disk: &Path
) -> Result<Vec<Node>> {
	Ok(
		read_dir(parent_on_disk).context("Could not read parent directory from disk")?
		.map(|x| x.context("Failed to look for next module on disk"))
		.collect::<Result<Vec<_>>>()?.into_iter()
		.map(|child| child.path())
		.map(|child| {
			let name = child.file_stem().and_then(|name| name.to_str()).context("File stem contains non-UTF8.")?;

			let modules = [
				// Inner module
				if child.is_dir() {
					LUAU_SOURCE_EXTENSIONS.into_iter()
					.map(|ex| child.join(format!("init.{ex}")))
					.find(|path| path.is_file())
					.map(|source| NodeModule {
						source_on_disk: source,
						dir_on_disk: Some(child.clone())
					})
				} else { None },

				// Outer module
				if child.is_dir() {
					LUAU_SOURCE_EXTENSIONS.into_iter()
					.map(|ex| parent_on_disk.join(format!("{name}.{ex}")))
					.find(|path| path.is_file())
					.map(|source| NodeModule {
						source_on_disk: source,
						dir_on_disk: Some(child.clone())
					})
				} else { None },

				// Leaf module
				if child.is_file() && name != "init" {
					if let Some(extension) = child.extension().and_then(|ex| ex.to_str()) {
						if LUAU_SOURCE_EXTENSIONS.into_iter().any(|ex| ex == &extension) {
							Some(NodeModule {
								source_on_disk: child.clone(),
								dir_on_disk: None
							})
						} else { None }
					} else { None }
				} else { None }
			].into_iter().filter_map(|x| x).collect::<Vec<_>>();

			if modules.len() > 1 {
				bail!(
					"The module at {} has more than one possible source file:\n\n{}",
					child.display(),
					modules.into_iter().enumerate()
					.map(|(index, module)| format!(" -> {}. {}", index + 1, module.source_on_disk.display()))
					.reduce(|a, b| format!("{a}\n{b}"))
					.unwrap_or_default()
				)
			} else if modules.is_empty() && child.is_file() {
				return Ok(None);
			}

			let module = modules.into_iter().next();

			Ok(Some(Node {
				parent_on_disk: parent_on_disk.to_owned(),
				name: name.to_string(),
				module,
				subnodes: if child.is_dir() {
					let subnodes = discover_nodes(&child).with_context(|| format!("Could not discover subnodes of {name}"))?;
					if subnodes.is_empty() { return Ok(None); }
					subnodes
				} else {
					[].into()
				}
			}))
		})
		.filter_map(|x| x.transpose())
		.collect::<Result<Vec<_>>>()?
	)
}

fn main() -> Result<()> {
    let modules = discover_nodes(
		&env::current_dir().context("Failed to get the working directory")?
	).context("Failed to discover Luau modules")?;
	dbg!(modules);

	// TODO: losslessly parse relative require paths in source code
	// TODO: move around source files on the filesystem
	// TODO: re-emit relative require paths to source code

	Ok(())
}
