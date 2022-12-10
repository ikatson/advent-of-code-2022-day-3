#include <iostream>
#include <fstream>
#include <vector>

#include "read_file.h"

namespace ad3p2
{
    std::vector<char> readFile(std::string_view name)
    {
        std::string filename(name);
        // Open the file in binary mode
        std::ifstream file(filename, std::ios::binary);
        if (!file.good())
        {
            throw std::runtime_error("error opening file");
        }

        // Create a vector to store the file data
        std::vector<char> buffer((std::istreambuf_iterator<char>(file)),
                                 std::istreambuf_iterator<char>());

        // Close the file
        file.close();
        return buffer;
    }
}
