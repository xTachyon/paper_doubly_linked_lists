#include "solutions.hpp"

StdList::StdList(size_t) {
}

void StdList::add(uint64_t element) {
    list.push_back(element);
}

uint64_t StdList::sum_all() const {
    uint64_t sum = 0;
    for (uint64_t i : list) {
        sum += i;
    }
    return sum;
}
