#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
    int n,x;
    cin >> n >> x;
    vector<ll> coins(n);
    for(int i=0;i<n;i++) cin >> coins[i];

    vector<ll> ways(x+1,0);
    ways[0] = 1;
    for(int i=0;i<n;i++) {
        for(int j=coins[i];j<=x;j++) {
            ways[j] += ways[j - coins[i]];
        }
    }

    ll res = ways[x]%1000000007;

    cout << res << "\n";
    return 0;
}