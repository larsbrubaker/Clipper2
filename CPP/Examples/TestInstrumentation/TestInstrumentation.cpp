#include <iostream>
#include <fstream>
#include <iomanip>
#include "clipper2/clipper.h"
using namespace Clipper2Lib;

void printPath(const std::string& name, const Path64& path) {
    std::cout << name << " (" << path.size() << " points): ";
    for (size_t i = 0; i < path.size(); ++i) {
        std::cout << "(" << path[i].x << "," << path[i].y << ")";
        if (i < path.size() - 1) std::cout << " ";
    }
    std::cout << std::endl;
}

void printPaths(const std::string& name, const Paths64& paths) {
    std::cout << name << " (" << paths.size() << " paths):" << std::endl;
    for (size_t i = 0; i < paths.size(); ++i) {
        std::cout << "  Path " << i << ": ";
        printPath("", paths[i]);
    }
}

int main() {
    std::cout << "=== C++ Clipper2 TestNegativeOrientation Instrumentation ===" << std::endl;

    Paths64 subjects, clips, solution;

    // Create the same paths as the test
    subjects.push_back(MakePath({ 0,0, 0,100, 100,100, 100,0 }));
    subjects.push_back(MakePath({ 10,10, 10,110, 110,110, 110,10 }));
    clips.push_back(MakePath({ 50,50, 50,150, 150,150, 150,50 }));

    std::cout << "\nInput paths:" << std::endl;
    printPaths("Subjects", subjects);
    printPaths("Clips", clips);

    std::cout << "\nOrientation checks:" << std::endl;
    std::cout << "IsPositive(subjects[0]): " << (IsPositive(subjects[0]) ? "true" : "false") << std::endl;
    std::cout << "IsPositive(subjects[1]): " << (IsPositive(subjects[1]) ? "true" : "false") << std::endl;
    std::cout << "IsPositive(clips[0]): " << (IsPositive(clips[0]) ? "true" : "false") << std::endl;

    std::cout << "\nArea calculations:" << std::endl;
    std::cout << "Area(subjects[0]): " << Area(subjects[0]) << std::endl;
    std::cout << "Area(subjects[1]): " << Area(subjects[1]) << std::endl;
    std::cout << "Area(clips[0]): " << Area(clips[0]) << std::endl;

    // Perform the union with negative fill rule
    std::cout << "\nPerforming Union with FillRule::Negative..." << std::endl;
    solution = Union(subjects, clips, FillRule::Negative);

    std::cout << "\nResult:" << std::endl;
    printPaths("Solution", solution);

    std::cout << "\nExpected: 1 path with 12 vertices" << std::endl;
    std::cout << "Actual: " << solution.size() << " path(s) with ";
    if (solution.size() > 0) {
        std::cout << solution[0].size() << " vertices in first path" << std::endl;
    } else {
        std::cout << "0 vertices (no paths)" << std::endl;
    }

    // Also test with other fill rules for comparison
    std::cout << "\n=== Comparison with other fill rules ===" << std::endl;

    Paths64 nonzero_result = Union(subjects, clips, FillRule::NonZero);
    std::cout << "\nNonZero result: " << nonzero_result.size() << " paths" << std::endl;
    if (nonzero_result.size() > 0) {
        printPaths("NonZero Solution", nonzero_result);
    }

    Paths64 evenodd_result = Union(subjects, clips, FillRule::EvenOdd);
    std::cout << "\nEvenOdd result: " << evenodd_result.size() << " paths" << std::endl;
    if (evenodd_result.size() > 0) {
        printPaths("EvenOdd Solution", evenodd_result);
    }

    Paths64 positive_result = Union(subjects, clips, FillRule::Positive);
    std::cout << "\nPositive result: " << positive_result.size() << " paths" << std::endl;
    if (positive_result.size() > 0) {
        printPaths("Positive Solution", positive_result);
    }

    return 0;
}