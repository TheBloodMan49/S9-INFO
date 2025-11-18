#include <bits/stdc++.h>
using namespace std;

int main() {
    int n, m;
    cin >> n >> m;

    vector<string> grid(n);
    for (int i = 0; i < n; i++) {
        cin >> grid[i];
    }

    int si = -1, sj = -1;
    vector<pair<int, int>> monsters;

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            if (grid[i][j] == 'A') {
                si = i;
                sj = j;
            } else if (grid[i][j] == 'M') {
                monsters.push_back({i, j});
            }
        }
    }

    vector<vector<int>> monster_dist(n, vector<int>(m, INT_MAX));
    queue<tuple<int, int, int>> q;

    for (auto [mi, mj] : monsters) {
        monster_dist[mi][mj] = 0;
        q.push({mi, mj, 0});
    }

    int dx[] = {0, 0, 1, -1};
    int dy[] = {1, -1, 0, 0};

    while (!q.empty()) {
        auto [i, j, d] = q.front();
        q.pop();

        for (int k = 0; k < 4; k++) {
            int ni = i + dx[k];
            int nj = j + dy[k];

            if (ni >= 0 && ni < n && nj >= 0 && nj < m &&
                grid[ni][nj] != '#' && monster_dist[ni][nj] > d + 1) {
                monster_dist[ni][nj] = d + 1;
                q.push({ni, nj, d + 1});
            }
        }
    }

    vector<vector<int>> player_dist(n, vector<int>(m, INT_MAX));
    vector<vector<pair<int, int>>> parent(n, vector<pair<int, int>>(m, {-1, -1}));

    player_dist[si][sj] = 0;
    q.push({si, sj, 0});

    char dir_char[] = {'R', 'L', 'D', 'U'};

    int ei = -1, ej = -1;

    while (!q.empty()) {
        auto [i, j, d] = q.front();
        q.pop();

        // Check if we reached boundary
        if (i == 0 || i == n - 1 || j == 0 || j == m - 1) {
            ei = i;
            ej = j;
            break;
        }

        for (int k = 0; k < 4; k++) {
            int ni = i + dx[k];
            int nj = j + dy[k];

            if (ni >= 0 && ni < n && nj >= 0 && nj < m &&
                grid[ni][nj] != '#' && player_dist[ni][nj] > d + 1 &&
                monster_dist[ni][nj] > d + 1) {
                player_dist[ni][nj] = d + 1;
                parent[ni][nj] = {i, j};
                q.push({ni, nj, d + 1});
            }
        }
    }

    if (ei == -1) {
        cout << "NO\n";
        return 0;
    }

    // Reconstruct path
    string path;
    int ci = ei, cj = ej;

    while (ci != si || cj != sj) {
        int pi = parent[ci][cj].first;
        int pj = parent[ci][cj].second;

        // Determine direction
        if (ci == pi + 1) path += 'D';
        else if (ci == pi - 1) path += 'U';
        else if (cj == pj + 1) path += 'R';
        else if (cj == pj - 1) path += 'L';

        ci = pi;
        cj = pj;
    }

    reverse(path.begin(), path.end());

    cout << "YES\n";
    cout << path.length() << "\n";
    if (!path.empty()) {
        cout << path << "\n";
    }

    return 0;
}