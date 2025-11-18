# 🧭 Fiche Mémo — Fonctions PromQL (Prometheus)

> Un résumé des fonctions essentielles de Prometheus pour l’observation, le monitoring et les requêtes dans Grafana ou Prometheus UI.

---

## ⚙️ 1. Fonctions de base sur les compteurs (`_total`)

| Fonction | Description | Exemple | Résultat |
|-----------|--------------|----------|-----------|
| `rate(v[5m])` | Taux de croissance par seconde d’un compteur sur 5 min | `rate(http_requests_total[5m])` | Requêtes/seconde |
| `irate(v[1m])` | Taux instantané (2 derniers points) | `irate(http_requests_total[1m])` | Taux quasi instantané |
| `increase(v[1h])` | Augmentation totale sur la période | `increase(http_requests_total[1h])` | Total sur 1h |

📘 **Astuce :**  
- `rate()` → pour les graphiques continus  
- `increase()` → pour des totaux

---

## 🧮 2. Fonctions d’agrégation

| Fonction | Description | Exemple | Résultat |
|-----------|--------------|----------|-----------|
| `sum()` | Somme les valeurs | `sum(rate(http_requests_total[5m]))` | Taux total |
| `avg()` | Moyenne des valeurs | `avg(rate(http_requests_total[5m]))` | Taux moyen |
| `min()` / `max()` | Valeur min / max | `max(cpu_usage)` | Max du CPU |
| `count()` | Compte les séries | `count(http_requests_total)` | Nombre de séries |
| `stddev()` / `stdvar()` | Écart-type / variance | `stddev(rate(http_requests_total[5m]))` | Variabilité |

💡 **Groupement :**
```promql
sum by (job, method)(rate(http_requests_total[5m]))
```

---

## ⏱️ 3. Fonctions temporelles

| Fonction | Description | Exemple | Résultat |
|-----------|--------------|----------|-----------|
| `delta(v[1h])` | Différence entre 1ère et dernière valeur | `delta(memory_usage_bytes[1h])` | Variation mémoire |
| `idelta(v[1m])` | Variation instantanée | `idelta(memory_usage_bytes[1m])` | Changement rapide |
| `changes(v[1h])` | Nombre de changements de valeur | `changes(up[1h])` | Nombre de transitions |
| `time()` | Temps actuel en secondes Unix | `time()` | Valeur unique |

---

## 📊 4. Fonctions mathématiques et de transformation

| Fonction | Description | Exemple |
|-----------|--------------|----------|
| `abs(v)` | Valeur absolue | `abs(node_filesystem_free_bytes)` |
| `ceil(v)` / `floor(v)` / `round(v)` | Arrondi | `ceil(cpu_usage)` |
| `clamp_min(v, x)` / `clamp_max(v, x)` | Force les bornes | `clamp_max(cpu_usage, 100)` |
| `label_replace(v, dst, repl, src, regex)` | Remplace un label via regex | `label_replace(metric, "env", "$1", "instance", "(prod.*)")` |
| `histogram_quantile(φ, rate(bucket[5m]))` | Calcule un quantile (p95, p99…) sur histogramme | `histogram_quantile(0.95, rate(request_duration_seconds_bucket[5m]))` |

---

## 🚦 5. Fonctions logiques et de comparaison

| Fonction | Description | Exemple |
|-----------|--------------|----------|
| `bool` | Comparaison logique (renvoie 1 ou 0) | `(cpu_usage > 0.9) bool` |
| `and`, `or`, `unless` | Combine des séries logiquement | `up == 0 or instance_down == 1` |
| `absent(v)` | Vérifie qu’une métrique est absente | `absent(up{job="api"})` |

---

## 📈 6. Fonctions statistiques et avancées

| Fonction | Description | Exemple |
|-----------|--------------|----------|
| `quantile(φ, v)` | Quantile sur valeurs instantanées | `quantile(0.9, http_request_duration_seconds)` |
| `topk(k, v)` | `k` plus grandes valeurs | `topk(5, rate(http_requests_total[5m]))` |
| `bottomk(k, v)` | `k` plus petites valeurs | `bottomk(3, rate(cpu_usage[5m]))` |

---

## 🧠 Exemple complet

```promql
sum by (method)(
  rate(hellocontroller_hello_get_total{
    application="tp-hello-monitoring",
    result="success"
  }[5m])
)
```
➡️ Taux de requêtes "hello" réussies par méthode sur 5 minutes.

---

## 📘 Rappels utiles

- Les intervalles (`[5m]`, `[1h]`) définissent la **fenêtre de calcul**.
- Les opérateurs `by()` / `without()` contrôlent les **labels conservés** après agrégation.
- Les compteurs (`_total`) s’utilisent avec `rate()` / `increase()`.
- Les gauges (valeurs instantanées) s’utilisent directement ou avec `delta()`.

---

