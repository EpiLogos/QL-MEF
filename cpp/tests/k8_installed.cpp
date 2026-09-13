// Compiled only against the installed public headers/library (no source includes).
#include <ql/coupled_clock.h>
#include <ql/m2_aperture.h>
#include <cassert>
#include <iostream>
#include <string>
int main() {
    QL_CoupledClock clock{{0, 0}, {1, 17}, {3, 9, 21}, {9, 8}, 1, {0, 0}, 4};
    assert(ql_clock_advance(&clock, 4, 80, &clock) == QL_CLOCK_OK);
    assert(clock.inscription.turns == 1 && clock.inscription.half_degrees == 0);
    assert(clock.lensing.turns == 1 && clock.lensing.half_degrees == 657);
    assert(std::string(ql_clock_centre()->source_ref) == "#3-5-5/0");
    assert(ql_m2_aperture_at(7)->reciprocal == ql_m2_aperture_at(8)->coordinate);
    assert(ql_m_live_node_by_id(ql_m2_aperture_at(7)->coordinate));
    std::cout << "installed C++ clock + current registry + 18-aperture consumer passed\n";
}
