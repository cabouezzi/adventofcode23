#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <utility>
#include <vector>

using namespace std;

const int ROWS = 140;
const int COLUMNS = 140;

vector<string> get_lines(ifstream *input) {
    vector<string> lines = {};
    string line;
    while (getline(*input, line)) {
        lines.push_back(line);
    }
    return lines;
}

string join(vector<string> lines, const string &delim) {
    string s = lines[0];
    for (int i = 1; i < lines.size(); i++) {
        s.append(delim + lines[i]);
    }
    return s;
}

enum Direction {
    NORTH = -ROWS, SOUTH = ROWS, EAST = 1, WEST = -1
};

class Graph {
private:
    string data;

public:
    int start = -1;

    set<int> vertices;

    explicit Graph(vector<string> lines);

    set<Direction> get_dirs(int index);

    vector<set<int>> bfs();
};

Graph::Graph(vector<string> lines) {
    data = join(std::move(lines), "");
    for (int i = 0; i < data.length(); i++) {
        if (data[i] == 'S') {
            start = i;
            break;
        }
    }
    assert(start != -1);
}

set<Direction> reach(char c) {
    switch (c) {
        case '|':
            return {NORTH, SOUTH};
        case '-':
            return {EAST, WEST};
        case 'L':
            return {NORTH, EAST};
        case 'J':
            return {NORTH, WEST};
        case '7':
            return {SOUTH, WEST};
        case 'F':
            return {EAST, SOUTH};
        case 'S':
            return {NORTH, EAST, SOUTH, WEST};
        case '.':
            return {};
        default:
            cout << "Unknown character '" << c << "'" << endl;
            return {};
    }
}

set<Direction> Graph::get_dirs(int index) {
    char c = data[index];
    set<Direction> dirs = {};
    for (Direction dir: reach(c)) {
        int target = index + dir;
        if (target < 0 || target >= data.length()) continue;
        auto compat = static_cast<Direction>(-dir);
        if (reach(data[target]).contains(compat)) {
            dirs.insert(dir);
        }
    }
    if (index / ROWS <= 0) dirs.erase(NORTH);
    if (index / ROWS >= ROWS - 1) dirs.erase(SOUTH);
    if (index % COLUMNS <= 0) dirs.erase(WEST);
    if (index % COLUMNS >= COLUMNS - 1) dirs.erase(EAST);
    return dirs;
}

vector<set<int>> Graph::bfs() {
    vector<set<int>> layers = {};
    set<int> discovered = {start};
    layers.push_back({start});
    vertices.insert(start);
    int i = 0;
    while (!layers[i].empty()) {
        layers.emplace_back();
        for (int vertex: layers[i]) {
            // loop through compatible indices
            set<Direction> dirs = get_dirs(vertex);
            for (Direction offset: dirs) {
                int target = vertex + offset;
                if (!discovered.contains(target)) {
                    discovered.insert(target);
                    layers[i + 1].insert(target);
                    vertices.insert(target);
                }
            }
        }
        i++;
    }
    layers.erase(layers.begin() + i);
    return layers;
}

int main() {
    ifstream input("input.txt");
    vector<string> lines = get_lines(&input);

    Graph graph = Graph(lines);
    cout << "Part 1 – Furthest Pipe: " << graph.bfs().size() - 1 << endl;

    long total = 0;
    set<char> wall_pipes = {'|', 'J', 'L'};
    for (int row = 0; row < lines.size(); row++) {
        int count = 0;
        int pipes = 0;
        for (int col = 0; col < lines[row].size(); col++) {
            if (graph.vertices.contains(row * COLUMNS + col)) {
                if (wall_pipes.contains(lines[row][col])) {
                    pipes += 1;
                }
            } else if (pipes % 2 == 1) {
                count++;
            }
        }
        total += count;
    }
    cout << "Part 2 – Area Enclosed: " << total << endl;

    return 0;
}