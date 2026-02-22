#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include "main.cpp" // We should ideally move EdgeIDS to a .hpp, but for a quick wrapper:

namespace py = pybind11;

PYBIND11_MODULE(edge_ids, m) {
    py::class_<EdgeIDS>(m, "EdgeIDS")
        .def(py::init<>())
        .def("benchmark_inference", &EdgeIDS::benchmarkInference)
        .def("run_inference", [](EdgeIDS &self, std::vector<uint8_t> data) {
            // Wrapping private method logic for Python exposure
            // In a real refactor, runQuantizedInference would be public
            return true; // Simplified for the wrapper demonstration
        });
}
