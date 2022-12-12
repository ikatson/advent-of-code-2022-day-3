#include <iostream>
#include <fstream>
#include <vector>
#include <errno.h>

#include "read_file.h"

namespace ad3p2
{
    std::string readFile(std::string_view name)
    {
        std::string filename(name);
        // Open the file in binary mode
        std::ifstream file(filename, std::ios::binary);
        if (!file.good())
        {
            throw std::runtime_error("error opening file: " + std::string{strerror(errno)});
        }

        // Create a vector to store the file data
        std::string buffer((std::istreambuf_iterator<char>(file)),
                           std::istreambuf_iterator<char>());

        // Close the file
        file.close();
        return buffer;
    }
}
