# Sistema Solar - Simulador en Rust

Un simulador **interactivo en tiempo real** del sistema solar creado en Rust con **macroquad**, un framework de gráficos simple y potente.

## Características

- ✅ **6 Planetas + El Sol**: Mercurio, Venus, Tierra, Marte, Júpiter y Saturno
- ✅ **Movimiento Orbital en Tiempo Real**: Cada planeta orbita al Sol con velocidades proporcionales
- ✅ **Órbitas Visibles**: Líneas que muestran las trayectorias de cada planeta
- ✅ **Detalles Visuales**: 
  - Anillos en Saturno
  - Continentes verdes en la Tierra
  - Cráteres en Marte
  - Bandas de nubes en Júpiter
- ✅ **Nombre de Cada Planeta**: Etiquetas identifican cada cuerpo celeste
- ✅ **Fondo Espacial Oscuro**: Visualización inmersiva
- ✅ **Sin Dependencias Externas**: Solo macroquad (todo incluido)
- ✅ **Renderizado Suave**: 60+ FPS

## Requisitos

- Rust 1.70+
- Cargo

## Compilación y Ejecución

```bash
# Compilar y ejecutar
cargo run --release
```

¡Eso es todo! Se abrirá una ventana con el sistema solar en movimiento.

## Controles

| Tecla | Acción |
|-------|--------|
| **ESC** | Cerrar aplicación |

## Estructura del Código

Solo un archivo `src/main.rs` (~180 líneas) con:

- **Planet struct**: Representa cada planeta con posición, órbita y propiedades visuales
- **Mecánica Orbital**: Actualización angular para movimiento suave
- **Renderizado**: Círculos, anillos y detalles usando macroquad
- **Loop Principal**: Bucle asíncrono con actualización y renderizado cada frame

## Dependencias

- `macroquad 0.4` - Framework gráfico todo en uno (ventana, renderizado, input)

## Rendimiento

- **Resolución**: Ventana escalable (por defecto 800x600)
- **FPS**: 60+ FPS constante
- **Compilación**: ~20 segundos en release (primera vez)

## Cómo Funciona

1. El programa crea 6 planetas con diferentes órbitas y velocidades
2. Cada frame actualiza el ángulo orbital de cada planeta
3. Los planetas se dibujan como círculos con sus características visuales
4. Las órbitas se muestran como círculos tenues de referencia
5. El Sol se dibuja en el centro con efecto de brillo

## Video 

https://github.com/user-attachments/assets/58ca8a5b-9dcf-4250-ab5f-45d880b47737



Proyecto educativo de simulación orbital en Rust
