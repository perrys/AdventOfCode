#pragma once
#include <cstdint>

namespace nr {

/**
 * Solve the simultaneous equations given in the coefficients vector (a), which represents an NxN
 * row-major matrix.
 *
 * The solution will be left in the targets vector (b).
 */
void guassJordanElimination(double* a, double* b, size_t n);

} // namespace nr
