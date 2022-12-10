
#include <chrono>
#include "impl.h"
#include "read_file.h"
#include <iostream>

template <typename F, typename V>
void bench(std::string_view name, F f, size_t warmupCount, size_t runCount, V expected_equal)
{
    for (auto i = 0; i < warmupCount; i++)
    {
        assert(f() == expected_equal);
    }

    auto begin = std::chrono::steady_clock::now();
    for (auto i = 0; i < runCount; i++)
    {
        f();
    }
    auto end = std::chrono::steady_clock::now();
    auto nanos = std::chrono::duration_cast<std::chrono::nanoseconds>(end - begin).count();
    std::cout << name << ": " << nanos / runCount << "ns per iteration" << std::endl;
}

int main()
{
    auto result = ad3p2::readFile("../input.txt");
    bench(
        "part1", [&]()
        { return ad3p2::processBuffer(result); },
        100000, 300000, 8085);
    bench(
        "part2", [&]()
        { return ad3p2::processBuffer_part2(result); },
        100000, 300000, 2515);
}