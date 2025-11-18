#include <bits/stdc++.h>
using namespace std;
#define ll long long

int B(int n, int k, int m) {
    if (n<0) return 0;
    if (k==0 && n != 0) return 0;
    if (n==0) return 1;

    int sum = 0;
    for (int i = 1; i<=m; i++) {
        sum += B(n-i,k-1,m);
    }
    return sum;

}

// NOT DONE
int main() {
    int n,k,m;

    while ( (cin >> n >> k >> m) ) {
        cout << B(n,k,m) << endl;
    }
    return 0;
}