#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <map>
#include <regex>

using namespace std;

int main() {
    ifstream input("input.txt");
    map<string, string> string2number = {
            {"one", "o1e"},
            {"two", "t2o"},
            {"three", "t3e"},
            {"four", "f4r"},
            {"five", "f5e"},
            {"six", "s6x"},
            {"seven", "s7n"},
            {"eight", "e8t"},
            {"nine", "n9e"},
    };

    long int total = 0;
    string line;
    while (getline(input, line)) {
        for (const auto &pair : string2number)
            line = regex_replace(line, regex(pair.first), pair.second);

        int first = -1;
        int last = -1;
        for (char c : line) {
            if (!isnumber(c)) continue;
            if (first == -1) first = c - '0';
            last = c - '0';
        }
        assert(first >= 0 && last >= 0);
        total += first * 10 + last;
    }
    cout << "Total: " << total << endl;
    return 0;
}
