#include <bits/stdc++.h>
using namespace std;

class union_find {
    vector<int> id;
    vector<int> sz;
    int count;
public:
    union_find(int N): id(N,0), sz(N,1) {
        count = N;
        for (int i=0; i<N; i++) {
            id[i]=i;
        }
    }

    int nb_components() {
        return count;
    }

    int size_set(int i) {
        return sz[find_set(i)];
    }

    int find_set(int i) {
        return ((id[i]==i) ? i : (id[i] = find_set(id[i])));
    }

    bool connected(int i, int j) {
        return find_set(i) == find_set(j);
    }

    void union_set(int p, int q) {
        int i = find_set(p);
        int j = find_set(q);
        if (i==j) return;
        if (sz[i] == sz[j]) { id[i] = j; sz[j]+=sz[i]; }
        else { id[j] = i; sz[i]+=sz[j]; }
        --count;
    }
};

int main() {
    int n_recp;
    cin >> n_recp;

    union_find ingredients(500000+1);

    int possible = 0;

    for (int i = 0; i<n_recp; i++) {
        int n_ings;
        cin >> n_ings;
        vector<int> ings(n_ings);
        for (int j = 0; j<n_ings; j++) {
            cin >> ings[j];
        }

        unordered_set<int> unique_subpotions;
        int sum_size = 0;

        for (int j = 0; j<n_ings; j++) {
            int root = ingredients.find_set(ings[j]);
            if (unique_subpotions.find(root) == unique_subpotions.end()) {
                unique_subpotions.insert(root);
                sum_size += ingredients.size_set(root);
            }
        }

        if (sum_size == n_ings) {
            possible++;
            for (int j = 1; j<n_ings; j++) {
                ingredients.union_set(ings[0], ings[j]);
            }
        }

    }

    cout << possible << endl;
    return 0;
}