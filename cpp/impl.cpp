#include "impl.h"
#include <cstring>
#include <string>
#include <optional>

namespace
{

    constexpr std::uint64_t ONE = 1;

    std::uint32_t trailingZeros(std::uint64_t v)
    {
        return __builtin_ctzl(v);
    }

    std::optional<size_t> memchrVec(std::string_view haystack, char needle, size_t offset)
    {
        // if (offset >= haystack.size())
        // {
        //     return std::nullopt;
        // }
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
            if (byte >= 'a')
            {
                priority = byte - 'a' + 1;
            }
            else
            {
                priority = byte - 'A' + 27;
            }
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
    std::uint32_t processBuffer(std::string_view buf)
    {
        auto result = 0;
        size_t offset = 0;
        while (const auto newLinePos = memchrVec(buf, '\n', offset))
        {
            result += processLine(buf.data() + offset, *newLinePos);
            offset += *newLinePos + 1;
        }
        return result;
    }

    std::uint32_t processBuffer_part2(std::string_view buf)
    {
        auto result = 0;
        size_t l1_start = 0;
        while (const auto l1_len_opt = memchrVec(buf, '\n', l1_start))
        {
            auto l1_len = l1_len_opt.value();
            auto l2_start = l1_start + l1_len + 1;
            auto l2_len = memchrVec(buf, '\n', l2_start).value();
            auto l3_start = l2_start + l2_len + 1;
            auto l3_len = memchrVec(buf, '\n', l3_start).value();

            auto l1 = compartment(buf.data() + l1_start, l1_len);
            auto l2 = compartment(buf.data() + l2_start, l2_len);
            auto l3 = compartment(buf.data() + l3_start, l3_len);

            result += trailingZeros(l1 & l2 & l3);
            l1_start = l3_start + l3_len + 1;
        }
        return result;
    }
}