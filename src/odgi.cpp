// src/odgi.cpp

#include "odgi_wrapper.hpp"
#include <fstream>
#include <string>
#include "odgi-ffi/src/lib.rs.h"

std::unique_ptr<OpaqueGraph> load_graph(rust::Str path) {
    auto odgi_graph = std::make_unique<odgi::graph_t>();
    std::ifstream in{std::string(path)};

    if (!in) {
        return nullptr;
    }
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