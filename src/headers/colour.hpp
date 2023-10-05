namespace Utility2048 {
enum class Colour {
    WHITE,
    GREEN,
    LIGHT_GREEN,
    MAGENTA,
    CYAN,
    RED,
    YELLOW
};
int getIntValue(Colour colour) {
    return static_cast<int>(colour);
}
}  // namespace Utility2048