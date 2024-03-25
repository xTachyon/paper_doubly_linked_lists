#include "solutions.hpp"

ManualList::ManualList(size_t) : head(nullptr), tail(nullptr), size(0) {
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
    if (!head) {
        head = newNode;
        tail = newNode;
    } else {
        tail->next = newNode;
        newNode->prev = tail;
        tail = newNode;
    }
    ++size;
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