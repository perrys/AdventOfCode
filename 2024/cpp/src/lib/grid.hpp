
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

struct CoOrdinate {
    size_t ix;
    size_t iy;
    bool operator==(const CoOrdinate& other) const {
        return this->ix == other.ix && this->iy == other.iy;
    }
    CoOrdinate move(const Direction& dir, const size_t nsteps = 1) const {
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

    std::optional<char> get(CoOrdinate xy) const;

    std::optional<char> getWithOffsets(CoOrdinate xy, Direction dxy) const;

    static Grid create(std::vector<std::string>&& lines);
};

} // namespace scp

namespace std {
template <> struct hash<scp::CoOrdinate> {
    size_t operator()(const scp::CoOrdinate& p) const {
        std::hash<size_t> hasher;
        size_t ix = p.ix >> 1 | p.ix << (sizeof(scp::CoOrdinate::ix) * 8 - 1);
        return hasher(ix) ^ hasher(p.iy);
    }
};
} // namespace std
