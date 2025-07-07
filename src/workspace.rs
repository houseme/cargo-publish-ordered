use crate::CARGO_TOML_DEFAULT;
use crate::error::Error;
use cargo_metadata::{Metadata, MetadataCommand, PackageId};
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Workspace {
    pub metadata: Metadata,
    pub publish_order: Vec<PackageId>,
}

impl Workspace {
    /// Load workspace metadata and support caching.
    pub fn load(manifest_path: Option<&str>) -> Result<Metadata, Error> {
        let manifest_path = manifest_path.unwrap_or(CARGO_TOML_DEFAULT);
        let cache_path = Path::new(".cargo_publish_ordered_cache.json");

        let manifest_mtime = fs::metadata(manifest_path).and_then(|m| m.modified()).ok();
        let cache_mtime = fs::metadata(cache_path).and_then(|m| m.modified()).ok();

        if let (Some(manifest_mtime), Some(cache_mtime)) = (manifest_mtime, cache_mtime) {
            if cache_mtime > manifest_mtime {
                if let Ok(cached) = fs::read_to_string(cache_path) {
                    if let Ok(metadata) = serde_json::from_str(&cached) {
                        return Ok(metadata);
                    }
                }
            }
        }

        let mut cmd = MetadataCommand::new();
        cmd.manifest_path(manifest_path);
        let metadata = cmd.exec()?;

        if let Ok(json) = serde_json::to_string(&metadata) {
            let _ = fs::write(cache_path, json);
        }

        Ok(metadata)
    }

    pub fn new(manifest_path: Option<&str>, exclude: &[String]) -> Result<Self, Error> {
        let metadata = Self::load(manifest_path)?;

        let mut graph = DiGraph::<PackageId, ()>::new();
        let mut package_indices = HashMap::new();

        for package in metadata.workspace_packages() {
            let idx = graph.add_node(package.id.clone());
            package_indices.insert(package.id.clone(), idx);
        }

        for package in metadata.workspace_packages() {
            let from_idx = package_indices[&package.id];
            for dep in &package.dependencies {
                if let Some(to_pkg) = metadata.packages.iter().find(|p| {
                    p.name.to_string() == dep.name && metadata.workspace_members.contains(&p.id)
                }) {
                    if let Some(&to_idx) = package_indices.get(&to_pkg.id) {
                        graph.add_edge(from_idx, to_idx, ());
                    }
                }
            }
        }

        let publish_order = toposort(&graph, None)
            .map_err(|_| Error::CyclicDependency)?
            .into_iter()
            .map(|idx| graph[idx].clone())
            .filter(|pkg_id| !exclude.contains(&metadata[pkg_id].name))
            .collect();

        Ok(Workspace {
            metadata,
            publish_order,
        })
    }

    pub fn packages_to_publish(&self) -> Vec<&cargo_metadata::Package> {
        self.publish_order
            .iter()
            .map(|pkg_id| &self.metadata[pkg_id])
            .collect()
    }
}
