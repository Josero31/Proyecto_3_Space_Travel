# Solar System Simulator - Rust Edition

Un simulador **interactivo en tiempo real** del sistema solar creado desde cero en Rust con un motor de renderizado por software personalizado.

## Características Principales

- ✅ **7 Cuerpos Celestes**: El Sol + Mercurio, Venus, Tierra, Marte, Júpiter y Saturno
- ✅ **Movimiento Orbital en Tiempo Real**: Cada planeta orbita al Sol con velocidades astronómicamente proporcionadas
- ✅ **Rotación Axial**: Los cuerpos celestes rotan sobre sus ejes
- ✅ **Cámara 3D Interactiva**: Navega libremente con teclado
- ✅ **Saltos Instantáneos**: Warp a cualquier planeta (teclas 1-6)
- ✅ **Renderizado por Software**: Motor personalizado sin dependencias de GPU
- ✅ **Rendimiento Optimizado**: 30+ FPS en resolución 1280x720
- ✅ **Sin Dependencias Externas Complejas**: Solo minifb para ventana

## Requisitos

- Rust 1.70+
- Cargo

## Compilación y Ejecución

```bash
# Compilar
cargo build --release

# Ejecutar
cargo run --release
```

¡Eso es todo! Se abrirá una ventana interactiva con el sistema solar en movimiento.

## Controles

### Movimiento de Cámara
| Tecla | Acción |
|-------|--------|
| **W** | Avanzar |
| **S** | Retroceder |
| **A** | Izquierda |
| **D** | Derecha |
| **Q** | Bajar |
| **E** | Subir |

### Saltos a Planetas
| Tecla | Destino |
|-------|---------|
| **1** | Sol |
| **2** | Mercurio |
| **3** | Venus |
| **4** | Tierra |
| **5** | Marte |
| **6** | Júpiter |

### Salida
- **ESC** - Cerrar aplicación

## Estructura del Código

El proyecto está en un solo archivo `src/main.rs` con ~500 líneas de código que incluye:

- **Vec3**: Vector 3D con operaciones (magnitud, normalización, dot, cross)
- **Camera**: Sistema de cámara en primera persona
- **CelestialBody**: Representación de planetas con órbitas y rotación
- **Proyección 3D → 2D**: Conversión perspectiva personalizada
- **Rasterización**: Círculos dibujados con algoritmo Midpoint

## Arquitectura Técnica

### Sistema de Coordenadas
- Plano eclíptico en Y=0
- Órbitas circulares alrededor del Sol en el origen
- Distancias y tamaños escalados para visualización óptima

### Física Orbital Simplificada
- Velocidades orbitales inversamente proporcionales a √r
- Movimiento tangencial suave alrededor del Sol
- Delta-time independiente para consistencia

### Renderizado
- Proyección perspectiva personalizada
- Rasterización de círculos (Midpoint Circle Algorithm)
- Buffer RGB 32-bit
- Culling por profundidad

## Dependencias

Solo una dependencia:
- `minifb 0.25` - Ventana y framebuffer

## Rendimiento

- **Resolución**: 1280x720
- **FPS**: 30+ FPS típico
- **Compilación**: ~2 segundos en release
