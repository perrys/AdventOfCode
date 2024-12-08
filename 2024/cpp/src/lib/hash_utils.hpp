#pragma once

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <utility>

namespace scp {

template <typename I> I rotateRight(I n, uint8_t nbits) {
    return n >> nbits | n << (sizeof(I) * 8 - nbits);
}

} // namespace scp

namespace std {
template <typename P1, typename P2> struct hash<std::pair<P1, P2>> {
    size_t operator()(const std::pair<P1, P2>& p) const {
        std::hash<P1> hash1;
        std::hash<P2> hash2;
        return hash1(p.first) ^ hash2(p.second);
    }
};
} // namespace std
