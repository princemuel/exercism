#include <stdint.h>

namespace targets {
class Alien {
 public:
  int32_t x_coordinate;
  int32_t y_coordinate;
  int32_t health;

  Alien(int32_t x, int32_t y) : x_coordinate(x), y_coordinate(y), health(3) {}

  int32_t get_health() const { return health; }

  bool hit() {
    if (health <= 0) return false;
    health--;
    return true;
  }

  bool is_alive() const { return health > 0; }

  bool teleport(int32_t x_new, int32_t y_new) {
    x_coordinate = x_new;
    y_coordinate = y_new;
    return true;
  }

  bool collision_detection(const Alien& other) const {
    return x_coordinate == other.x_coordinate &&
           y_coordinate == other.y_coordinate;
  }
};

}  // namespace targets
