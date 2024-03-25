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

class ManualList {
    struct Node {
        uint64_t data;
        Node* next;
        Node* prev;

        Node(uint64_t d, Node* p = nullptr, Node* n = nullptr) : data(d), prev(p), next(n) {
        }
    };

    Node* head;
    Node* tail;

  public:
    ManualList(size_t nodes);
    ~ManualList();
    void add(uint64_t element);
    uint64_t sum_all() const;
};