
#include <charconv>
#include <optional>
#include <sstream>

namespace scp {

template <typename T> class Parser {
  public:
    static auto parse(const char* begin, const char* end) -> std::optional<T> {
        T result{};
        auto [ptr, ec] = std::from_chars(begin, end, result);
        if (ec == std::errc() && ptr == end) {
            return result;
        }
        return {};
    }
    template <typename I> auto operator()(const I& subrange) const -> T {
        T result{};
        auto [ptr, ec] = std::from_chars(&*subrange.begin(), &*subrange.end(), result);
        if (ec == std::errc()) {
            return result;
        }
        std::stringstream msg;
        msg << "ERROR: invalid integer \"" << ptr << "\", "
            << std::make_error_condition(ec).message();
        throw std::runtime_error(msg.str());
    }
};

using parseInt = Parser<int>;

} // namespace scp
