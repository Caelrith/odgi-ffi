// src/odgi_wrapper.hpp

#pragma once
#include "odgi.hpp"
#include "rust/cxx.h"
#include <memory>

struct OpaqueGraph {
    std::unique_ptr<odgi::graph_t> graph;
};

std::unique_ptr<OpaqueGraph> load_graph(rust::Str path);
const odgi::graph_t& get_graph_t(const OpaqueGraph& graph);
uint64_t get_node_count(const odgi::graph_t& graph);