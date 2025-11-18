#include <vector>
#include <iostream>
#include <algorithm>

using namespace std;

int main() {
    int n,t;
    cin >> n >> t;

    vector<int> machines(n,0);
    for (int i = 0; i < n; i++) {
        cin >> machines[i];
    }

//    for (int i = 0; i < n; i++) {
//    cout << machines[i] << " ";
//    }
//    cout << endl;

    long long low = 0, high = 1e18, mid;
    while (low < high) {
        mid = low + (high - low) / 2;
        long long total = 0;

        for (int i = 0; i < n; i++) {
            total += mid / machines[i];
            if (total >= t) break; // Early exit if we already meet the requirement
        }

        if (total >= t) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    cout << low << endl;
    
    return 0;
}