1. What is the birthPlace of Zendaya ?

```sparql
SELECT DISTINCT ?x
WHERE { dbr:Zendaya dbo:birthPlace ?x . }
```

1. Which films are starring Zendaya ?

```sparql
SELECT DISTINCT ?x
WHERE { ?x dbo:starring dbr:Zendaya }
```

1. Which television show (dbo :TelevisionShow) have for channel Disney_Channel ?

```sparql
SELECT DISTINCT ?x
WHERE { ?x a dbo:TelevisionShow. ?x dbo:channel dbr:Disney_Channel }
```

1. Which elements, of type "Actor", have a "birthPlace" which is "Scotland" or somewhere in Scotland ?

```sparql
SELECT DISTINCT ?x
WHERE { {?x a dbo:Actor; dbo:birthPlace dbr:Scotland}
  UNION {?x a dbo:Actor; dbo:birthPlace ?b. ?b dbo:country dbr:Scotland} }
```

1. How many films did Thomas_Langmann produce ?

```sparql
SELECT count(?x)
WHERE { ?x dbo:producer dbr:Thomas_Langmann}
```

1. How many films did Jennifer Lawrence star in ?

```sparql
SELECT (count(?f) as ?count)
WHERE { 
  ?f dbo:starring dbr:Jennifer_Lawrence.
}
```

1. Which mountain is the highest after the Mont Blanc (help : dbo :elevation) ?

```sparql
SELECT ?x
WHERE { ?x a dbo:Mountain. ?x dbo:elevation ?h. dbr:Mont_Blanc dbo:elevation ?mb. FILTER(?h < ?mb)}
ORDER BY DESC(?h)
LIMIT 1
```

1. Who produced the most films ?

```sparql
SELECT ?p (count(?f) as ?count)
WHERE { 
  ?f a dbo:Film; dbo:producer ?p.
}
GROUP BY ?p
ORDER BY DESC(?count)
LIMIT 1
```

1. Was Natalie Portman born in the United States (Help : you have to check if the town where
Natalie Portman was born is in the United States) ?

```sparql
ASK
WHERE { dbr:Natalie_Portman dbo:birthPlace ?p. ?p dbo:country dbr:United_States}
```

1. Which countries have more than two official languages ?

```sparql
SELECT ?c (count(?l) as ?count)
WHERE { 
  ?c a dbo:Country; dbo:officialLanguage ?l.
}
GROUP BY ?c
HAVING (count(?l) > 2)
```

1. Which countries are the location of more than two caves (dbo :Cave) ?

```sparql
SELECT ?p (count(?c) as ?count)
WHERE { 
  ?c a dbo:Cave; dbo:location ?p. ?p a dbo:Country.
}
GROUP BY ?p
HAVING (count(?c) > 2)
```

1. Is Jeanne Calment still alive (dbo :deathDate) ?

```sparql
ASK
WHERE { 
  FILTER NOT EXISTS{dbr:Jeanne_Calment dbo:deathDate ?x.}
}
```

1. Is Jackie Chan still alive (dbo :deathDate) ?

```sparql
ASK
WHERE { 
  FILTER NOT EXISTS{dbr:Jackie_Chan dbo:deathDate ?x.}
}
```
