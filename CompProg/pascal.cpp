#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
  ll N;
  cin >> N;
  if (N == 1) {
    cout << 0 << '\n';
    return 0;
  }
  ll p = 0;
  if (N % 2 == 0) p = 2;
  else {
    for (ll i = 3; i * i <= N; i += 2) {
      if (N % i == 0) { p = i; break; }
    }
  }
  if (p == 0) p = N;
  cout << (N - N / p) << '\n';
  return 0;
}

