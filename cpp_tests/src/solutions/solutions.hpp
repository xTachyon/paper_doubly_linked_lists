#pragma once

#include <cstdint>
#include <list>

class StdList {
    std::list<uint64_t> list;
public:
    StdList(size_t nodes);
    void add(uint64_t element);
    uint64_t sum_all() const;
};