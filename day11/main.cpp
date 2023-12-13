#include <iostream>
#include <fstream>
#include <set>

using namespace std;

typedef struct {
    long row;
    long col;
} Coord;

typedef struct {
    vector<vector<bool>> matrix;
    vector<Coord> galaxy_indices;
    vector<int> empty_rows;
    vector<int> empty_columns;
} Data;

Data read_input() {
    ifstream input("input.txt");
    vector<vector<bool>> matrix;
    vector<Coord> galaxies;
    vector<int> empty_rows;
    vector<int> empty_columns;

    int row = 0;
    string line;
    while (getline(input, line)) {
        vector<bool> row_data = {};
        for (char col: line)
            row_data.push_back(col == '#');
        matrix.push_back(row_data);
        if (none_of(row_data.begin(), row_data.end(), [](bool e) { return e; }))
            empty_rows.push_back(row);
        row++;
    }

    for (int col = 0; col < matrix[0].size(); col++) {
        bool all_empty = true;
        for (vector<bool> &r: matrix) {
            if (r[col]) {
                all_empty = false;
                break;
            }
        }
        if (all_empty) {
            empty_columns.push_back(col);
        }
    }

    for (int r = 0; r < matrix.size(); r++)
        for (int col = 0; col < matrix[r].size(); col++)
            if (matrix[r][col])
                galaxies.push_back({.row = r, .col = col});

    return {.matrix = matrix,
            .galaxy_indices = galaxies,
            .empty_rows = empty_rows,
            .empty_columns = empty_columns};
}

long diff(Coord lhs, Coord rhs, const Data &data, long scale_factor) {
    long scaled = 0;

    for (int col: data.empty_columns)
        if (min(lhs.col, rhs.col) < col && col < max(lhs.col, rhs.col))
            scaled += scale_factor - 1;

    for (int row: data.empty_rows)
        if (min(lhs.row, rhs.row) < row && row < max(lhs.row, rhs.row))
            scaled += scale_factor - 1;

    return abs(rhs.row - lhs.row) + abs(rhs.col - lhs.col) + scaled;
}

int main() {
    Data data = read_input();

    long long total_1 = 0;
    for (int i = 0; i < data.galaxy_indices.size() - 1; i++)
        for (int j = i + 1; j < data.galaxy_indices.size(); j++)
            total_1 += diff(data.galaxy_indices[i], data.galaxy_indices[j], data, 2);
    cout << "Part 1: " << " " << total_1 << endl;

    long long total_2 = 0;
    for (int i = 0; i < data.galaxy_indices.size() - 1; i++)
        for (int j = i + 1; j < data.galaxy_indices.size(); j++)
            total_2 += diff(data.galaxy_indices[i], data.galaxy_indices[j], data, 1000000);
    cout << "Part 2: " << " " << total_2 << endl;

    return 0;
}
