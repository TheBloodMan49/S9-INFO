#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
  string data;
  cin >> data;

  int possible = 1;
  if (data.length() > 26) possible = 0;
  else {
    unordered_set<char> set;
    for(int i = 0; i < data.length(); i++) {
      if (set.find(data[i]) == set.end()) {
        set.insert(data[i]);
      } else {
        possible = 0;
      }
    }
  }

  cout << possible << endl;
  return 0;
}