#include <iostream>

#include "read_file.h"
#include "impl.h"

int main(int argc, const char **argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("you need to provide a filename");
    }
    std::string filename(argv[1]);
    auto s = ad3p2::readFile(filename);

    auto result = ad3p2::processBuffer(s);
    std::cout << result << std::endl;
}