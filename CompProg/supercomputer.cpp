#include <bits/stdc++.h>
using namespace std;
#define ll long long

void F(vector<int>& mem, int bit) {
    mem[bit-1] ^= 1;
}

void C(vector<int>& mem, int bit0, int bit1) {
    cout << accumulate(mem.begin() + bit0 - 1, mem.begin() + bit1, 0) << "\n";
}

// WARN : NOT DONE
int main() {
    int n,k;
    cin >> n >> k;

    vector<int> mem(n,0);
    vector<string> op;

    for (int i = 0; i < k; i++) {
        string s;
        cin >> s;
        op.push_back(s);
    }

    for (auto o : op) {

    }
}