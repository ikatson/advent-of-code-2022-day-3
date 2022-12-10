#include "impl.h"
#include <cstring>
#include <string>
#include <optional>

namespace
{

    constexpr std::uint64_t ONE = 1;

    std::uint32_t trailingZeros(std::uint64_t v)
    {
        std::uint32_t result = 0;
        while ((v & 1) == 0)
        {
            result += 1;
            v = v >> 1;
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
            auto byte = data[i];
            std::uint8_t priority = 0;
            if (byte >= 'a' && byte <= 'z')
            {
                priority = byte - 'a' + 1;
            }
            else if (byte >= 'A' && byte <= 'Z')
            {
                priority = byte - 'A' + 27;
            }
            assert(priority > 0);
            result |= ONE << priority;
        }
        return result;
    }

    std::uint32_t processLine(const char *data, size_t sz)
    {
        size_t mid = sz >> 1;
        std::uint64_t left = compartment(data, mid);
        std::uint64_t right = compartment(data + mid, mid);
        auto result = trailingZeros(left & right);
        return result;
    }
}

namespace ad3p2
{
    std::uint32_t processBuffer(const std::vector<char> &buf)
    {
        assert(compartment("a", 1) == 0b10);
        assert(compartment("d", 1) == 0b10000);
        assert(compartment("A", 1) == 1 << 27);

        auto result = 0;
        size_t offset = 0;
        while (const auto newLinePos = memchrVec(buf, '\n', offset))
        {
            if (newLinePos == 0)
            {
                break;
            }

            result += processLine(buf.data() + offset, *newLinePos);
            offset += *newLinePos + 1;
        }
        return result;
    }
}