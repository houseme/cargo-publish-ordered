use cargo_metadata::{Metadata, MetadataCommand, PackageId};
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use std::collections::HashMap;

use crate::error::Error;

pub struct Workspace {
    pub metadata: Metadata,
    pub publish_order: Vec<PackageId>,
}

impl Workspace {
    pub fn new(manifest_path: Option<&str>, exclude: &[String]) -> Result<Self, Error> {
        // Get metadata
        let mut cmd = MetadataCommand::new();
        if let Some(path) = manifest_path {
            cmd.manifest_path(path);
        }
        let metadata = cmd.exec()?;

        // Build dependency graph
        let mut graph = DiGraph::<PackageId, ()>::new();
        let mut package_indices = HashMap::new();

        // Add node (package in workspace)
        for package in metadata.workspace_packages() {
            let idx = graph.add_node(package.id.clone());
            package_indices.insert(package.id.clone(), idx);
        }

        // Add edges (dependencies)
        for package in metadata.workspace_packages() {
            let from_idx = package_indices[&package.id];
            for dep in &package.dependencies {
                if let Some(to_pkg) = metadata
                    .packages
                    .iter()
                    .find(|p| p.name == dep.name && metadata.workspace_members.contains(&p.id))
                {
                    if let Some(&to_idx) = package_indices.get(&to_pkg.id) {
                        graph.add_edge(from_idx, to_idx, ());
                    }
                }
            }
        }

        // Topological sorting
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
