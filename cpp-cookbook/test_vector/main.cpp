#include<vector>
#include<iostream>
#include<chrono>

void insert_1000000() {
    std::vector<int> v;
    for (int i = 0; i < 1000000; ++i) {
        v.push_back(i);
    }
}

void insert_1000000_with_capactiy() {
    std::vector<int> v;
    v.reserve(1000000);
    for (int i = 0; i < 1000000; ++i) {
        v.push_back(i);
    }
}




int main() {
#if defined(VERSIONNNNN)
    std::cout << "VERSIONNNNN: " << VERSIONNNNN << std::endl;
#endif
    // auto start = std::chrono::system_clock::now();
    // for (int i = 0; i < 100; i++) insert_1000000();
    // auto end = std::chrono::system_clock::now();
    // std::cout << "Elapsed time: " << std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count() << "ms\n";

    // start = std::chrono::system_clock::now();
    // for (int i = 0; i < 100; i++) insert_1000000_with_capactiy();
    // end = std::chrono::system_clock::now();
    // std::cout << "Elapsed time: " << std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count() << "ms\n";
}