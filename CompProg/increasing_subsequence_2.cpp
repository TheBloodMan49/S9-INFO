#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
    int n;
    while ( (cin >> n) && n != 0 ) {
        vector<ll> a(n);
        for (int i = 0; i < n; ++i) cin >> a[i];

        vector<ll> tails;            // values of smallest tail for each length
        vector<int> tail_idx;        // index in a of that tail
        vector<int> prev(n, -1);     // prev index in a for reconstruction

        for (int i = 0; i < n; ++i) {
            ll x = a[i];
            int pos = int(lower_bound(tails.begin(), tails.end(), x) - tails.begin());
            int prev_idx = (pos > 0) ? tail_idx[pos - 1] : -1;

            if (pos == (int)tails.size()) {
                tails.push_back(x);
                tail_idx.push_back(i);
            } else {
                tails[pos] = x;
                tail_idx[pos] = i;
            }
            prev[i] = prev_idx;
        }

        int len = (int)tails.size();
        cout << len;
        if (len > 0) {
            vector<ll> lis;
            int idx = tail_idx[len - 1];
            for (; idx != -1; idx = prev[idx]) lis.push_back(a[idx]);
            reverse(lis.begin(), lis.end());
            for (ll v : lis) cout << " " << v;
        }
        cout << "\n";
    }
    return 0;
}
