#pragma once

#include "lib/hash_utils.hpp"

#include <iostream>
#include <memory>
#include <optional>
#include <vector>

namespace scp {
struct Direction {
    int dx;
    int dy;
    bool operator==(const Direction& other) const {
        return this->dx == other.dx && this->dy == other.dy;
    }
    Direction operator+(const Direction& other) const {
        return {this->dx + other.dx, this->dy + other.dy};
    }
    Direction opposite() const {
        return {this->dx * -1, this->dy * -1};
    }
};

extern const Direction NORTH;
extern const Direction SOUTH;
extern const Direction EAST;
extern const Direction WEST;

template <typename T> struct GenCoordinate {
    T ix;
    T iy;
    bool operator==(const GenCoordinate& other) const {
        return this->ix == other.ix && this->iy == other.iy;
    }
    GenCoordinate move(const Direction& dir, const size_t nsteps = 1) const {
        return {this->ix + dir.dx * nsteps, this->iy + dir.dy * nsteps};
    }
    std::pair<int64_t, int64_t> displacement(const GenCoordinate& other) const {
        return {(int64_t)other.ix - (int64_t)this->ix, (int64_t)other.iy - (int64_t)this->iy};
    }
};

using Coordinate = GenCoordinate<size_t>;

class Grid {
  private:
    std::vector<std::string> rows;
    size_t rowWidth;

  public:
    Grid(std::vector<std::string>&& g) : rows(g), rowWidth(g[0].size()) {
    }

    size_t width() const {
        return this->rowWidth;
    }
    size_t height() const {
        return this->rows.size();
    }

    std::optional<char> get(Coordinate xy) const;

    void set(Coordinate xy, char c);
    std::optional<char> getWithOffsets(Coordinate xy, Direction dxy) const;

    template <typename F> std::optional<Coordinate> search(F predicate) {
        for (size_t iy = 0; iy < this->height(); ++iy) {
            for (size_t ix = 0; ix < this->width(); ++ix) {
                if (predicate(this->rows.at(iy).at(ix))) {
                    return {{ix, iy}};
                }
            }
        }
        return {};
    }

    void print() const;

    static Grid create(std::vector<std::string>&& lines);
};

} // namespace scp

namespace std {
template <typename T> struct hash<scp::GenCoordinate<T>> {
    size_t operator()(const scp::GenCoordinate<T>& p) const {
        std::hash<size_t> hasher;
        return hasher(scp::rotateRight(p.ix, 1)) ^ hasher(p.iy);
    }
};

template <> struct hash<scp::Direction> {
    size_t operator()(const scp::Direction& p) const {
        std::hash<size_t> hasher;
        return hasher(scp::rotateRight(p.dx, 1)) ^ hasher(p.dy);
    }
};

} // namespace std

std::ostream& operator<<(std::ostream& out, const scp::Coordinate& c);
std::ostream& operator<<(std::ostream& out, const scp::Direction& c);
