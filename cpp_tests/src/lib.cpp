#include <cstring>
#include "api.hpp"
#include "solutions/solutions.hpp"

#ifdef _WIN32
#    define EXPORT __declspec(dllexport)
#else
#    define EXPORT
#endif

extern "C" {
EXPORT RawLoadTestsResult load_tests();
}

template <typename T>
RawTestData sol(const char* name) {
    auto create = [](uintptr_t nodes) -> Handle {
        auto ptr = new T(nodes);
        return ptr;
    };
    auto destroy = [](Handle handle) {
        auto ptr = static_cast<T*>(handle);
        delete ptr;
    };
    auto add = [](Handle handle, uint64_t element) {
        auto ptr = static_cast<T*>(handle);
        ptr->add(element);
    };
    auto sum_all = [](Handle handle) -> uint64_t {
        auto ptr = static_cast<T*>(handle);
        return ptr->sum_all();
    };

    return RawTestData{ reinterpret_cast<const unsigned char*>(name), strlen(name), create, destroy, add, sum_all };
}

EXPORT RawLoadTestsResult load_tests() {
    static RawTestData TESTS[] = { sol<StdList>("std_list") };
    return RawLoadTestsResult{ TESTS, sizeof(TESTS) / sizeof(*TESTS) };
}
