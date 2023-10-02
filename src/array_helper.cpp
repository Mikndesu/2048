#include "headers/array_helper.hpp"

std::array<int, 4> ArrayHelper::getCertainColumn(const int column, std::array<std::array<int, 4>, 4>& array) {
    return array[column];
}

void ArrayHelper::setCertainColumn(const int column, std::array<int, 4>& column_array, std::array<std::array<int, 4>, 4>& array) {
    array[column] = column_array;
}

std::array<int, 4> ArrayHelper::getCertainRow(const int row, std::array<std::array<int, 4>, 4>& array) {
    std::array<int, 4> arr = {0, 0, 0, 0};
    for(int i = 0; i < 4; ++i) {
        arr[i] = array[i][row];
    }
    return arr;
}

void ArrayHelper::setCertainRow(const int row, std::array<int, 4>& row_array, std::array<std::array<int, 4>, 4>& array) {
    for(int i = 0; i < 4; ++i) {
        array[i][row] = row_array[i];
    }
}