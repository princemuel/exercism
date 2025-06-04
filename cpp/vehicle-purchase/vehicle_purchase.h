#pragma once

#include <string>

namespace vehicle_purchase
{

    bool needs_license(std::string kind);
    std::string choose_vehicle(std::string a, std::string b);
    double calculate_resell_price(double original_price, double age);

} // namespace vehicle_purchase
