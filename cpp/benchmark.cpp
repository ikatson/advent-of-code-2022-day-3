
#include <chrono>
#include "impl.h"
#include "read_file.h"
#include <iostream>

int main()
{
    auto result = ad3p2::readFile("../input.txt");
    auto warmupCount = 100000;
    auto count = 300000;

    for (auto i = 0; i < warmupCount; i++)
    {
        assert(ad3p2::processBuffer(result) == 8085);
    }

    auto begin = std::chrono::steady_clock::now();
    for (auto i = 0; i < count; i++)
    {
        ad3p2::processBuffer(result);
    }
    auto end = std::chrono::steady_clock::now();
    auto nanos = std::chrono::duration_cast<std::chrono::nanoseconds>(end - begin).count();
    std::cout << nanos / count << "ns per iteration" << std::endl;
}