# Space Travel - Solar System Simulator

Un simulador de sistema solar con renderer de software creado en Rust. Explora un universo procedural con planetas orbitando, rotación axial y una cámara libre para navegar por el espacio.

## Características Implementadas

- **Renderer de Software**: Implementación personalizada de rasterización y rendering 3D
- **Simulación Física**: Órbitas planetarias basadas en mecánica orbital
- **7 Cuerpos Celestes**: Un sol y 6 planetas con propiedades únicas
- **Cámara Libre**: Movimiento en todas las direcciones sobre el plano eclíptico
- **Warp Instantáneo**: Teletransportación rápida a diferentes planetas (teclas 1-6)
- **Rotación Planetaria**: Los planetas rotan sobre su eje a diferentes velocidades
- **Visualización 3D**: Proyección en perspectiva con depth buffering

## Requisitos

- Rust 1.70 o superior
- Cargo
- Windows/Linux/macOS con soporte para GPU (se usa wgpu internamente)

## Instalación

```bash
git clone <repositorio>
cd Proyecto_3_Space_Travel
cargo build --release
```

## Ejecución

```bash
cargo run --release
```

La ventana se abrirá mostrando el sistema solar. El simulador comienza con la cámara posicionada para ver todo el sistema.

## Controles

### Movimiento
- **W** - Avanzar
- **A** - Mover a la izquierda
- **S** - Retroceder
- **D** - Mover a la derecha
- **Q** - Subir
- **E** - Bajar

### Warping a Planetas
- **1** - Mercury
- **2** - Venus
- **3** - Earth
- **4** - Mars
- **5** - Jupiter
- **6** - Saturn

## Estructura del Proyecto

```
src/
├── main.rs           # Punto de entrada, loop de eventos y renderización
├── math/
│   └── mod.rs        # Matemática vectorial 3D (Vec3, Mat4, Quaternion)
├── renderer/
│   └── mod.rs        # Software renderer con rasterización
├── scene/
│   └── mod.rs        # Definición de cuerpos celestes
├── camera/
│   └── mod.rs        # Sistema de cámara con proyección
└── physics/
    └── mod.rs        # Funciones de física orbital
```

## Sistema Solar Incluido

El simulador incluye los siguientes cuerpos celestes:

| Nombre | Tipo | Órbita | Color | Tamaño |
|--------|------|--------|-------|--------|
| Sun | Estrella | Centro | Oro | Grande |
| Mercury | Planeta | 150 unidades | Marrón | Pequeño |
| Venus | Planeta | 250 unidades | Naranja | Mediano |
| Earth | Planeta | 400 unidades | Azul | Mediano |
| Mars | Planeta | 550 unidades | Rojo | Pequeño |
| Jupiter | Planeta | 900 unidades | Marrón/Oro | Muy Grande |
| Saturn | Planeta | 1200 unidades | Oro Pálido | Grande |

## Rendimiento

El simulador mantiene una velocidad de fotogramas apropiada (20-60 FPS dependiendo del hardware). 
Las principales optimizaciones incluyen:

- Depth buffer para evitar sobredibujo
- Proyección en perspectiva eficiente
- Rasterización de figuras geométricas optimizada

## Tecnologías Utilizadas

- **Rust**: Lenguaje de programación
- **Winit**: Gestión de ventanas y eventos
- **Pixels**: Backend de renderización 2D
- **Image**: Conversión de formatos de imagen

## Video de Demostración

[Insertar enlace a video de demostración aquí]

## Futuras Mejoras

- [ ] Anillos planetarios (Saturn)
- [ ] Lunas orbitando planetas
- [ ] Efectos visuales mejorados (colisión con objetos)
- [ ] Sistema de partículas para efectos de movimiento
- [ ] Música de fondo ambientale
- [ ] Zoom de cámara
- [ ] Estadísticas en tiempo real

## Créditos

Proyecto desarrollado para un curso de gráficos por computadora.

## Licencia

Este proyecto está disponible bajo la licencia MIT.
