#pragma once
#include <array>

class ArrayHelper {
   public:
    std::array<int, 4> getCertainRow(const int row, std::array<std::array<int, 4>, 4>& array);
    std::array<int, 4> getCertainColumn(const int column, std::array<std::array<int, 4>, 4>& array);
    void setCertainRow(const int row, std::array<int, 4>& row_array, std::array<std::array<int, 4>, 4>& array);
    void setCertainColumn(const int column, std::array<int, 4>& column_array, std::array<std::array<int, 4>, 4>& array);
};