#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
    int n,m;
    cin >> n >> m;
    vector<vector<int>> grid(n, vector<int>(m));
    vector<vector<int>> visited(n, vector<int>(m, -1));

    int x_end = n - 1;
    int y_end = m - 1;

    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        for (int j = 0; j < m; j++) {
            grid[i][j] = s[j] - '0';
        }
    }

    queue <pair<int, int>> q;
    q.push({0, 0});
    visited[0][0] = 0;
    while (!q.empty()) {
        pair<int, int> cell = q.front();
        q.pop();
        int x = cell.first;
        int y = cell.second;
        int jump = grid[x][y];
        int steps = visited[x][y];
        if (x == x_end && y == y_end) {
            cout << steps << endl;
            return 0;
        }
        // Jump down
        if (x + jump < n && visited[x + jump][y] == -1) {
            visited[x + jump][y] = steps+1;
            q.push({x + jump, y});
        }
        // Jump right
        if (y + jump < m && visited[x][y + jump] == -1) {
            visited[x][y + jump] = steps+1;
            q.push({x, y + jump});
        }
        // Jump up
        if (x - jump >= 0 && visited[x - jump][y] == -1) {
            visited[x - jump][y] = steps + 1;
            q.push({x - jump, y});
        }
        // Jump left
        if (y - jump >= 0 && visited[x][y - jump] == -1) {
            visited[x][y - jump] = steps + 1;
            q.push({x, y - jump});
        }
    }
    cout << -1 << endl;
    return 0;
}