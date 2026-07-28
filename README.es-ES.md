# tuidict

Diccionario TUI con soporte para múltiples pares de idiomas de FreeDict.

## Características

- Soporte multi-idioma - Descarga y usa cualquier diccionario de la base de datos de FreeDict
- Búsqueda en vivo mientras escribes con resultados instantáneos
- Búsqueda rápida por prefijo mediante estructura de datos Trie (búsquedas O(k))
- Almacenamiento en caché binario para tiempos de inicio instantáneos
- Descargas y gestión de diccionarios dentro de la aplicación

## Inicio Rápido

```bash
cargo install tuidict
```

1. Inicia la aplicación
2. Presiona `3` para ir a la página de Descargas
3. Explora los diccionarios disponibles y presiona `Enter` para descargar
4. Presiona `1` para volver a la página de Traducción
5. ¡Empieza a escribir para buscar!

## Atajos de Teclado

### Globales
- `1` - Página de traducción
- `2` - Página de gestión de diccionarios
- `3` - Página de descarga de diccionarios
- `q` o `Ctrl+C` - Salir

### Página de Traducción (Página 1)
- Escribir para buscar (resultados en vivo)
- `Tab` - Alternar entre diccionarios activos
- `j/k` o `↑/↓` - Navegar por los resultados
- `Ctrl+n/Ctrl+p` - Navegar por los resultados mientras editas
- `Enter` - Entrar en modo normal
- `Esc` - Volver al modo de edición
- `/` - Borrar entrada y volver al modo de edición

### Gestión de Diccionarios (Página 2)
- `j/k` o `↑/↓` - Navegar por la lista de diccionarios
- `Space` o `Enter` - Alternar diccionario activo/inactivo
- `d` - Eliminar diccionario (borra los archivos)

### Página de Descargas (Página 3)
- Escribir para filtrar diccionarios (búsqueda en vivo)
- `Ctrl+n/Ctrl+p` - Navegar mientras filtras
- `j/k` o `↑/↓` - Navegar por la lista de diccionarios
- `Esc` - Entrar en modo normal (detiene la edición del filtro)
- `Enter` - Descargar e instalar el diccionario seleccionado
- `/` - Borrar filtro y entrar en modo de edición

## Almacenamiento

- Configuración: `~/.config/tuidict/config.json`
- Diccionarios: `~/.local/share/tuidict/dictionaries/`
- Archivos de caché: Almacenados junto a los archivos del diccionario para una carga rápida

## Fuente de los Diccionarios

Los diccionarios provienen de [FreeDict](https://freedict.org/), un proyecto de diccionarios gratuito y de código abierto que soporta numerosos pares de idiomas.
