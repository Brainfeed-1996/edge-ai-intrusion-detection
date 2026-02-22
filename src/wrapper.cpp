#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <stdexcept>
#include "main.cpp" 

namespace py = pybind11;

PYBIND11_MODULE(edge_ids, m) {
    py::register_exception<std::runtime_error>(m, "RuntimeError");

    py::class_<EdgeIDS>(m, "EdgeIDS")
        .def(py::init<>())
        .def("benchmark_inference", &EdgeIDS::benchmarkInference)
        .def("run_inference", [](EdgeIDS &self, std::vector<uint8_t> data) {
            if (data.empty()) {
                throw std::runtime_error("Empty packet data received");
            }
            // Logic exposure
            return true; 
        });
}
