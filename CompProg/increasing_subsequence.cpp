#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
    int n;
    cin >> n;
    vector<ll> sequence(n);
    for (int i = 0; i < n; ++i) cin >> sequence[i];

    // patience sorting / tails method: O(n log n)
    vector<ll> tails;
    for (int i = 0; i < n; ++i) {
        ll x = sequence[i];
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }

    cout << tails.size() << "\n";
    return 0;
}