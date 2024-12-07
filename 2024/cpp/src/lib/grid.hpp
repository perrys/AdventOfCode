
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
};

extern const Direction NORTH;
extern const Direction SOUTH;
extern const Direction EAST;
extern const Direction WEST;

struct Coordinate {
    size_t ix;
    size_t iy;
    bool operator==(const Coordinate& other) const {
        return this->ix == other.ix && this->iy == other.iy;
    }
    Coordinate move(const Direction& dir, const size_t nsteps = 1) const {
        return {this->ix + dir.dx * nsteps, this->iy + dir.dy * nsteps};
    }
};

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

    static Grid create(std::vector<std::string>&& lines);
};

} // namespace scp

namespace std {
template <> struct hash<scp::Coordinate> {
    size_t operator()(const scp::Coordinate& p) const {
        std::hash<size_t> hasher;
        size_t ix = p.ix >> 1 | p.ix << (sizeof(scp::Coordinate::ix) * 8 - 1);
        return hasher(ix) ^ hasher(p.iy);
    }
};

template <> struct hash<scp::Direction> {
    size_t operator()(const scp::Direction& p) const {
        std::hash<size_t> hasher;
        size_t dx = p.dx >> 1 | p.dx << (sizeof(scp::Direction::dx) * 8 - 1);
        return hasher(dx) ^ hasher(p.dy);
    }
};

template <typename P1, typename P2> struct hash<std::pair<P1, P2>> {
    size_t operator()(const std::pair<P1, P2>& p) const {
        std::hash<P1> hash1;
        std::hash<P2> hash2;
        return hash1(p.first) ^ hash2(p.second);
    }
};

} // namespace std

std::ostream& operator<<(std::ostream& out, const scp::Coordinate& c);
