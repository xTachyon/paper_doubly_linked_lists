#include "solutions.hpp"

ManualList::ManualList(size_t) {
    auto node = new Node{ 0, nullptr, nullptr };
    head = node;
    tail = node;
}

ManualList::~ManualList() {
    while (head) {
        Node* temp = head;
        head = head->next;
        delete temp;
    }
}

void ManualList::add(uint64_t element) {
    Node* newNode = new Node(element);
    tail->next = newNode;
    newNode->prev = tail;
    tail = newNode;
}

uint64_t ManualList::sum_all() const {
    uint64_t sum = 0;
    Node* current = head;
    while (current) {
        sum += current->data;
        current = current->next;
    }
    return sum;
}