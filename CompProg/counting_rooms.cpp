#include <bits/stdc++.h>
using namespace std;
#define ll long long

int main() {
  int n,m;
  cin>>n>>m;
  vector<string> grid(n);
  for(int i=0;i<n;i++) {
    cin>>grid[i];
  }

  vector<vector<bool>> visited(n,vector<bool>(m,false));
  int room_count=0;
  vector<int> dir_x={0,0,1,-1};
  vector<int> dir_y={1,-1,0,0};

  for(int i=0;i<n;i++) {
    for(int j=0;j<m; j++) {
      if(grid[i][j] == '.' && !visited[i][j]) {
        room_count++;

        queue<pair<int,int>> to_visit;
        to_visit.push({i,j});
        while(!to_visit.empty()) {
          auto sq = to_visit.front();
          to_visit.pop();
          int x = sq.first, y = sq.second;
          if(visited[x][y]) continue;
          visited[x][y] = true;
            for(int d=0;d<4;d++) {
              int new_x = x + dir_x[d];
              int new_y = y + dir_y[d];
              if (new_x >= 0 && new_x < n && new_y >= 0 && new_y < m) {
                if (grid[new_x][new_y] == '.' && !visited[new_x][new_y]) {
                  to_visit.push({new_x, new_y});
                }
              }
            }
        }
      }
    }
  }
  cout<<room_count<<endl;
    return 0;
}