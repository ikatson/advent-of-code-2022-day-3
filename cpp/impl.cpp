#include "impl.h"
#include <cstring>

#include <iostream>
#include <string>
#include <optional>

namespace
{

    std::uint32_t trailingZeros(std::uint64_t v)
    {
        if (v == 0)
        {
            return 64;
        }
        std::uint32_t result = 0;
        while ((v & 1) == 0)
        {
            result += 1;
            v >>= 1;
        }
        return result;
    }

    std::optional<size_t> memchrVec(const std::vector<char> &haystack, char needle, size_t offset)
    {
        if (offset >= haystack.size())
        {
            return std::nullopt;
        }
        auto *start = haystack.data() + offset;
        auto *r = memchr(start, needle, haystack.size() - offset);
        if (r == nullptr)
        {
            return std::nullopt;
        }
        return static_cast<const char *>(r) - start;
    }

    std::uint64_t compartment(const char *data, size_t sz)
    {
        std::uint64_t result = 0;
        for (auto i = 0; i < sz; i++)
        {
            auto byte = static_cast<std::uint8_t>(data[i]);
            std::uint8_t priority = 0;
            if (byte >= 'a' && byte <= 'z')
            {
                priority = byte - 'a' + 1;
            }
            else if (byte >= 'A' && byte <= 'Z')
            {
                priority = byte - 'A' + 27;
            }
            else
            {
                throw std::runtime_error("unexpected");
            }
            result |= 1 << priority;
        }
        return result;
    }

    std::uint32_t processLine(const char *data, size_t sz)
    {
        size_t mid = sz >> 1;
        auto left = compartment(data, mid);
        auto right = compartment(data + mid, mid);
        auto result = trailingZeros(left & right);
        std::string line{data, sz};
        std::cout << line << " = " << result << std::endl;
        return result;
    }
}

namespace ad3p2
{
    std::uint32_t processBuffer(const std::vector<char> &buf)
    {
        auto result = 0;
        size_t offset = 0;
        while (const auto newLine = memchrVec(buf, '\n', offset))
        {
            if (newLine == 0)
            {
                break;
            }

            result += processLine(buf.data() + offset, *newLine - 1);
            offset += *newLine + 1;
        }
        return result;
    }
}