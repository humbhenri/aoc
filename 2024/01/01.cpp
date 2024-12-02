#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <unordered_map>
#include <iterator>

/**
 * @brief This function takes a filename as an argument and reads the two column
 *        file, returning the sum of the absolute differences between the two
 *        columns.
 *
 * @return The sum of the absolute differences between the two columns.
 */
auto part1(const std::string &filename) {
    std::string line;
    std::ifstream file(filename);
    std::vector<int> v1;
    std::vector<int> v2;
    int i1;
    int i2;
    if (file.is_open()) {
        while (getline(file, line)) {
            std::istringstream iss(line);
            iss >> i1;
            v1.push_back(i1);
            iss >> i2;
            v2.push_back(i2);
        }
    }
    std::sort(v1.begin(), v1.end());
    std::sort(v2.begin(), v2.end());
    auto sum = 0;
    for (int i = 0; i < v1.size(); i++) {
        sum += std::abs(v1[i] - v2[i]);
    }
    return sum;
}

/**
 * @brief This function takes a filename as an argument and reads the two column
 *        file, returning the score. The score is calculated by multiplying the
 *        value of the number in the first column by the number of times it appears in the other column.
 *
 * @return The score.
 */
auto part2(const std::string &filename) {
    std::string line;
    std::ifstream file(filename);
    std::vector<int> v1;
    std::vector<int> v2;
    int i1;
    int i2;
    if (file.is_open()) {
        while (getline(file, line)) {
            std::istringstream iss(line);
            iss >> i1;
            v1.push_back(i1);
            iss >> i2;
            v2.push_back(i2);
        }
    }
    auto score = 0;
    for (auto i : v1) {
        score += i * std::count(v2.begin(), v2.end(), i);
    }
    return score;
}

int main() {
    std::cout << part1("01.input") << std::endl;
    std::cout << part2("01.input") << std::endl;
    return 0;
}