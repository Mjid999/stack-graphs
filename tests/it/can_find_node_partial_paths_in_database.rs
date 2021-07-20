// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use std::collections::HashSet;

use stack_graphs::arena::Handle;
use stack_graphs::graph::Node;
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::PartialPath;
use stack_graphs::partial::PartialPaths;
use stack_graphs::stitching::Database;

use crate::test_graphs;

fn check_node_partial_paths(
    graph: &mut StackGraph,
    node: Handle<Node>,
    expected_partial_paths: &[&str],
) {
    let file = graph[node].file().unwrap();
    let mut partials = PartialPaths::new();
    let mut database = Database::new();
    partials.find_all_partial_paths_in_file(graph, file, |graph, partials, path| {
        if !path.is_complete_as_possible(graph) {
            return;
        }
        if !path.is_productive(partials) {
            return;
        }
        database.add_partial_path(graph, partials, path);
    });

    let mut results = Vec::<Handle<PartialPath>>::new();
    database.find_candidate_partial_paths_from_node(node, &mut results);

    let actual_partial_paths = results
        .into_iter()
        .map(|path| database[path].display(graph, &mut partials).to_string())
        .collect::<HashSet<_>>();
    let expected_partial_paths = expected_partial_paths
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
    assert_eq!(actual_partial_paths, expected_partial_paths);
}

#[test]
fn class_field_through_function_parameter() {
    let mut fixture = test_graphs::class_field_through_function_parameter::new();
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.main_bar,
        &[
            "<> () [main.py(10) reference bar] -> [root] <a.foo()/[main.py(7)].bar> ()",
            "<> () [main.py(10) reference bar] -> [root] <b.foo()/[main.py(7)].bar> ()",
        ],
    );
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.a_x_ref,
        &["<> () [a.py(8) reference x] -> [a.py(14) definition x] <> ()"],
    );
    // no references in b.py
}

#[test]
fn cyclic_imports_python() {
    let mut fixture = test_graphs::cyclic_imports_python::new();
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.main_foo,
        &["<> () [main.py(6) reference foo] -> [root] <a.foo> ()"],
    );
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.a_b,
        &["<> () [a.py(6) reference b] -> [root] <b> ()"],
    );
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.b_a,
        &["<> () [b.py(8) reference a] -> [root] <a> ()"],
    );
}

#[test]
fn cyclic_imports_rust() {
    let mut fixture = test_graphs::cyclic_imports_rust::new();
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.main_FOO,
        &[
            "<> () [test.rs(101) reference FOO] -> [test.rs(204) definition BAR] <> ()",
            "<> () [test.rs(101) reference FOO] -> [test.rs(304) definition FOO] <> ()",
        ],
    );
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.b_BAR,
        &["<> () [test.rs(305) reference BAR] -> [test.rs(204) definition BAR] <> ()"],
    );
}

#[test]
fn sequenced_import_star() {
    let mut fixture = test_graphs::sequenced_import_star::new();
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.main_foo,
        &["<> () [main.py(6) reference foo] -> [root] <a.foo> ()"],
    );
    check_node_partial_paths(
        &mut fixture.graph,
        fixture.a_b,
        &["<> () [a.py(6) reference b] -> [root] <b> ()"],
    );
    // no references in b.py
}