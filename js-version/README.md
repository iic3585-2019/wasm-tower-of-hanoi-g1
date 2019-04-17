# Torre de Hanoi 
## js version
Este proyecto usa `node` y `yarn`.

## Setup

Clona el repo e instala las dependencias usando el siguiente comando:

```
yarn
```

## Ejecución

Para correr el programa debes usar:
```
yarn start
```

Luego visita `localhost:8080`.

## Estructura del código

El archivo `src/index.js` permite la interacción con la interfaz.

El algoritmo de resolución del problema lo puedes encontrar en la carpeta `src/hanoi`.

El HTML se encuentra en `stc/static/index.html`.

## Algoritmo
El algoritmo tiene complejidad de O(2^n), siendo n el número de discos. El programa empieza a notar una diferencia significativa a partir de los 15 discos y ya aproximadamente en los 28 se empieza a demorar 5 segundos.