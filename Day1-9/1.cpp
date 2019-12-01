#include <fstream>
#include <iostream>
#include <vector>
using namespace std;

int fuelReq(int m)
{
    return (m / 3) - 2;
}

int part1(vector<int> &modules)
{
    int out = 0;
    for (auto m : modules)
    {
        out += fuelReq(m);
    }
    return out;
}

int part2(vector<int> &modules)
{
    int out = 0;
    for (auto m : modules)
    {
        int f = fuelReq(m);
        while (f > 0)
        {
            out += f;
            f = fuelReq(f);
        }
    }
    return out;
}

int main()
{
    vector<int> modules;

    ifstream infile("1.txt");
    string line;
    while (getline(infile, line))
    {
        modules.push_back(stoi(line));
    }

    cout << "Part 1: " << part1(modules) << "\n";
    cout << "Part 2: " << part2(modules) << "\n";

    return 0;
}
