#include "darts.h"
#include <cmath>

namespace darts
{

    int score(double x, double y)
    {
        double point = sqrt(x * x + y * y);

        if (point > 10)
            return 0;
        if (point > 5)
            return 1;
        if (point > 1)
            return 5;

        return 10;
    }

}
