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
    int num_cases;
    cin >> num_cases;
    
    for (int i = 0; i<num_cases; i++) {
        int num_lines;
        int cpt = 0;
        cin >> num_lines;
        
        union_find friend_graph(num_lines+1);
        map<string,int> names;
        
        for (int j = 0; j<num_lines; j++) {
            string name1, name2;
            cin >> name1 >> name2;
            if(names.find(name1) == names.end()) names[name1] = cpt++;
            if(names.find(name2) == names.end()) names[name2] = cpt++;
            int p = names[name1];
            int q = names[name2];
            friend_graph.union_set(p,q);
            cout << friend_graph.size_set(p) << endl;
        }
    }
    return 0;
}