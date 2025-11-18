#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
    int n;
    cin >> n;
    vector<vector<string>> grid(n, vector<string>(n));

    char c;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            cin >> c;
            grid[i][j] = c;
        }
    }

//    for(int i = 0; i < n; i++) {
//        for(int j = 0; j < n; j++) {
//            cout << grid[i][j];
//        }
//        cout << "\n";
//    }

    vector<vector<ll>> dp(n, vector<ll>(n, 0));
    if(grid[0][0] != "*") dp[0][0] = 1;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            if (grid[i][j] == "*") continue;
            if (i > 0) dp[i][j] += dp[i-1][j];
            if (j > 0) dp[i][j] += dp[i][j-1];
            dp[i][j] %= 1000000007;
        }
    }
    cout << dp[n-1][n-1] << "\n";
    return 0;
}