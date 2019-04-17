# Torres de Hanoi
En este proyecto buscamos analizar el tiempo de ejecución del algoritmo para resolver el problema de las torres de hanoi. El objetivo es comparar el rendimiento usando Rust + wasm y en JS normal.

## Algoritmo
El algoritmo tiene complejidad de O(2^n), siendo n el número de discos. El programa empieza a notar una diferencia significativa a partir de los 15 discos y ya aproximadamente en los 28 se empieza a demorar 5 segundos. Después cada vez que se aumenta un disco, se duplica el tiempo de ejecución del algoritmo.