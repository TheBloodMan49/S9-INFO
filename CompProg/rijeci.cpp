#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
  int k;
  cin >> k;
  int n_a = 1;
  int n_b = 0;

  for (int i = 0; i < k; i++) {
    int new_a = n_b;
    int new_b = n_a + n_b;
    n_a = new_a;
    n_b = new_b;
  }

  cout << n_a << " " << n_b << "\n";
  return 0;
}