#include <bits/stdc++.h>
using namespace std;
#define ll long long

int reverse_number(int num) {
  int reversed = 0;
  while (num > 0) {
    reversed = reversed * 10 + num % 10;
    num /= 10;
  }
  return reversed;
}

int main() {
  int n1, n2;
  cin >> n1 >> n2;

  int rev_n1 = reverse_number(n1);
  int rev_n2 = reverse_number(n2);

  if (rev_n1 > rev_n2) {
    cout << rev_n1 << "\n";
  } else {
    cout << rev_n2 << "\n";
  }
}