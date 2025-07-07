use crate::CARGO_TOML_DEFAULT;
use crate::error::Error;
use std::fs;
use toml_edit::{DocumentMut, Table};

/// Sort or check dependencies in `Cargo.toml`.
pub fn sort_dependencies(
    path: &str,
    check: bool,
    workspace: bool,
    descending: bool,
    verbose: bool,
) -> Result<(), Error> {
    let manifest_path = if path.is_empty() {
        CARGO_TOML_DEFAULT
    } else {
        path
    };
    if verbose {
        println!("[INFO] Reading manifest file: {path}");
    }
    let content = fs::read_to_string(manifest_path).map_err(|e| Error::FileRead {
        path: path.to_string(),
        source: e,
    })?;
    if verbose {
        println!("[INFO] Successfully read manifest file.");
    }

    let mut doc = content
        .parse::<DocumentMut>()
        .map_err(|e| Error::TomlParse {
            path: path.to_string(),
            source: e,
        })?;
    if verbose {
        println!("[INFO] Successfully parsed TOML content.");
    }

    let mut is_sorted = true;
    let sections = ["dependencies", "dev-dependencies", "build-dependencies"];

    for &section_name in &sections {
        if let Some(table) = doc.get_mut(section_name).and_then(|i| i.as_table_mut()) {
            if verbose {
                println!("[INFO] Processing section: [{section_name}]");
            }
            is_sorted &= check_and_sort_table(table, descending, verbose);
        }
    }

    if workspace {
        if let Some(ws_deps) = doc
            .get_mut("workspace")
            .and_then(|i| i.as_table_mut())
            .and_then(|t| t.get_mut("dependencies"))
            .and_then(|i| i.as_table_mut())
        {
            if verbose {
                println!("[INFO] Processing section: [workspace.dependencies]");
            }
            is_sorted &= check_and_sort_table(ws_deps, descending, verbose);
        }
    }

    if check {
        if is_sorted {
            println!("[INFO] Dependencies in `{path}` are correctly sorted.");
            Ok(())
        } else {
            Err(Error::NotSorted {
                path: path.to_string(),
            })
        }
    } else {
        if verbose {
            println!("[INFO] Writing changes back to: {path}");
        }
        fs::write(path, doc.to_string()).map_err(|e| Error::FileWrite {
            path: path.to_string(),
            source: e,
        })?;
        println!("[INFO] Successfully sorted dependencies in `{path}`.");
        Ok(())
    }
}

/// Check and sort a single dependency table to return whether the table is already ordered.
fn check_and_sort_table(table: &mut Table, descending: bool, verbose: bool) -> bool {
    let original_order: Vec<_> = table.iter().map(|(k, _)| k.to_string()).collect();

    if verbose {
        println!("[DEBUG] Original order: {original_order:?}");
    }

    if descending {
        table.sort_values_by(|k1, _v1, k2, _v2| k2.cmp(k1));
    } else {
        table.sort_values();
    }

    let new_order: Vec<_> = table.iter().map(|(k, _)| k.to_string()).collect();
    if verbose {
        println!("[DEBUG] New order: {new_order:?}");
    }

    let sorted = original_order == new_order;
    if verbose {
        println!("[INFO] Section sorted: {sorted}");
    }
    sorted
}
