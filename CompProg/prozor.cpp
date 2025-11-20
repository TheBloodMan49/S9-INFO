#include <bits/stdc++.h>
using namespace std;
#define ll long long

int flies_in(vector<string>& window, int r, int s, int x, int y, int k) {
  int count = 0;
  for (int i = x+1; i < x + k-1; i++) {
    for (int j = y+1; j < y + k-1; j++) {
      if (window[i][j] == '*') {
        count++;
      }
    }
  }
  return count;
}

int main() {
  int r,s,k;
  cin >> r >> s >> k;
  vector<string> window;
  for (int i = 0; i < r; i++) {
    string row;
    cin >> row;
    window.push_back(row);
  }

  int max_flies = 0;
  int max_x = 0;
  int max_y = 0;
  for (int i = 0; i <= r - k; i++) {
    for (int j = 0; j <= s - k; j++) {
      int current_flies = flies_in(window, r, s, i, j, k);
      if (current_flies > max_flies) {
        max_flies = current_flies;
        max_x = i;
        max_y = j;
      }
    }
  }

  cout << max_flies << "\n";
  for (int i = 0; i < r; i++) {
    for (int j = 0; j < s; j++) {
      if ((i == max_x && j == max_y) || (i == max_x && j == max_y+k-1) || (i == max_x+k-1 && j == max_y) || (i == max_x+k-1 && j == max_y+k-1)) {
        cout << "+";
      } else if ((i == max_x && (j < max_y+k-1 && j > max_y)) || (i == max_x+k-1 && (j < max_y+k-1 && j > max_y))) {
        cout << "-";
      }else if ((j == max_y && (i < max_x+k-1 && i > max_x)) || (j == max_y+k-1 && (i < max_x+k-1 && i > max_x))) {
        cout << "|";
      } else {
        cout << window[i][j];
      }
    }
    cout << endl;
  }
  return 0;

}  