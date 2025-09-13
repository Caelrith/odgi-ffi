// File: src/odgi.cpp
#include "odgi_wrapper.hpp"
#include <fstream>
#include <string>
#include <vector>
#include "odgi-ffi/src/lib.rs.h"

// --- Core API ---
std::unique_ptr<OpaqueGraph> load_graph(rust::Str path) {
    auto odgi_graph = std::make_unique<odgi::graph_t>();
    std::ifstream in{std::string(path)};
    if (!in) { return nullptr; }
    odgi_graph->deserialize(in);
    auto wrapper = std::make_unique<OpaqueGraph>();
    wrapper->graph = std::move(odgi_graph);
    return wrapper;
}

const odgi::graph_t& get_graph_t(const OpaqueGraph& wrapper) {
    return *wrapper.graph.get();
}

uint64_t get_node_count(const odgi::graph_t& graph) {
    return graph.get_node_count();
}

// --- Query Functions ---
rust::Vec<rust::String> graph_get_path_names(const odgi::graph_t& graph) {
    rust::Vec<rust::String> names;
    graph.for_each_path_handle([&](const odgi::path_handle_t& path) {
        names.push_back(graph.get_path_name(path));
    });
    return names;
}

std::unique_ptr<odgi::PathPosition> graph_project(const odgi::graph_t& graph, rust::Str path_name, uint64_t pos) {
    if (!graph.has_path(std::string(path_name))) {
        return nullptr;
    }
    odgi::path_handle_t path = graph.get_path_handle(std::string(path_name));
    
    // CORRECTED: The method `get_path_length` does not exist in this odgi version.
    // We calculate the path length manually by summing the lengths of its nodes.
    uint64_t path_len = 0;
    graph.for_each_step_in_path(path, [&](const odgi::step_handle_t& step) {
        path_len += graph.get_length(graph.get_handle_of_step(step));
        return true;
    });

    if (pos >= path_len) {
        return nullptr;
    }

    uint64_t current_pos = 0;
    std::unique_ptr<odgi::PathPosition> found_pos = nullptr;

    graph.for_each_step_in_path(path, [&](const odgi::step_handle_t& step) {
        if (found_pos) { // If we've already found it, just let the iterator finish
            return true;
        }

        odgi::handle_t handle = graph.get_handle_of_step(step);
        uint64_t node_len = graph.get_length(handle);

        if (pos < current_pos + node_len) {
            uint64_t offset_in_step = pos - current_pos;
            
            found_pos = std::make_unique<odgi::PathPosition>(odgi::PathPosition{
                (uint64_t)graph.get_id(handle),
                graph.get_is_reverse(handle) ? (node_len - 1 - offset_in_step) : offset_in_step,
                !graph.get_is_reverse(handle)
            });
        }
        current_pos += node_len;
        return true;
    });

    return found_pos;
}

rust::String graph_get_node_sequence(const odgi::graph_t& graph, uint64_t node_id) {
    if (!graph.has_node(node_id)) return "";
    return graph.get_sequence(graph.get_handle(node_id, false));
}

uint64_t graph_get_node_len(const odgi::graph_t& graph, uint64_t node_id) {
    if (!graph.has_node(node_id)) return 0;
    return graph.get_length(graph.get_handle(node_id, false));
}

rust::Vec<odgi::Edge> graph_get_successors(const odgi::graph_t& graph, uint64_t node_id) {
    rust::Vec<odgi::Edge> edges;
    if (!graph.has_node(node_id)) return edges;

    auto handle_fwd = graph.get_handle(node_id, false);
    graph.follow_edges(handle_fwd, false, [&](const odgi::handle_t& next) {
        edges.push_back({(uint64_t)graph.get_id(next), true, !graph.get_is_reverse(next)});
        return true;
    });
    
    auto handle_rev = graph.get_handle(node_id, true);
    graph.follow_edges(handle_rev, false, [&](const odgi::handle_t& next) {
        edges.push_back({(uint64_t)graph.get_id(next), false, !graph.get_is_reverse(next)});
        return true;
    });
    return edges;
}

rust::Vec<odgi::Edge> graph_get_predecessors(const odgi::graph_t& graph, uint64_t node_id) {
    rust::Vec<odgi::Edge> edges;
    if (!graph.has_node(node_id)) return edges;

    auto handle_fwd = graph.get_handle(node_id, false);
    graph.follow_edges(handle_fwd, true, [&](const odgi::handle_t& prev) {
        edges.push_back({(uint64_t)graph.get_id(prev), !graph.get_is_reverse(prev), true});
        return true;
    });

    auto handle_rev = graph.get_handle(node_id, true);
    graph.follow_edges(handle_rev, true, [&](const odgi::handle_t& prev) {
        edges.push_back({(uint64_t)graph.get_id(prev), !graph.get_is_reverse(prev), false});
        return true;
    });
    return edges;
}

rust::Vec<rust::String> graph_get_paths_on_node(const odgi::graph_t& graph, uint64_t node_id) {
    rust::Vec<rust::String> paths;
    if (!graph.has_node(node_id)) return paths;

    auto handle = graph.get_handle(node_id, false);
    graph.for_each_step_on_handle(handle, [&](const odgi::step_handle_t& step) {
        paths.push_back(graph.get_path_name(graph.get_path_handle_of_step(step)));
        return true;
    });
    return paths;
}