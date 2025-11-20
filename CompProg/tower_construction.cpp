#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
  int n;
  cin >> n;
  int towers = 0;
  int width;
  int last_width = -1;
  for (int i = 1; i <= n; i++) {
    cin >> width;
    if (width > last_width) {
      towers++;
    }
    last_width = width;
  }
  cout << towers << "\n";
  return 0;
}