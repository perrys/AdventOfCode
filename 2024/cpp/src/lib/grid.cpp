
#include "grid.hpp"

#include <iostream>

namespace scp {

const Direction NORTH{0, -1};
const Direction SOUTH{0, 1};
const Direction EAST{1, 0};
const Direction WEST{-1, 0};

Grid Grid::create(std::vector<std::string>&& lines) {
    size_t width = 0;
    for (size_t i = 0; i < lines.size(); ++i) {
        const auto& line = lines[i];
        if (i > 1) {
            if (width != line.length()) {
                std::cerr << "ERROR: inconsistent line length at " << i << std::endl;
                return Grid({});
            }
        } else {
            width = line.length();
        }
    }
    return Grid(std::move(lines));
}
std::optional<char> Grid::get(Coordinate c) const {
    if (c.ix < this->rowWidth && c.iy < this->rows.size()) {
        return this->rows.at(c.iy).at(c.ix);
    }
    return {};
}

void Grid::set(Coordinate c, char p) {
    this->rows.at(c.iy).at(c.ix) = p;
}

std::optional<char> Grid::getWithOffsets(Coordinate c, Direction d) const {
    if (c.ix == 0 && d.dx < 0) {
        return {};
    }
    if (c.iy == 0 && d.dy < 0) {
        return {};
    }
    return this->get({c.ix + d.dx, c.iy + d.dy});
}
} // namespace scp

std::ostream& operator<<(std::ostream& out, const scp::Coordinate& c) {
    out << "(" << c.ix << "," << c.iy << ")";
    return out;
}
