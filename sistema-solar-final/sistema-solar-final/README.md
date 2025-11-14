# 🌌 Sistema Solar Interactivo 3D

**Autor:** Pablo Cabrera  
**Carné:** 231156  
**Curso:** Gráficas por Computadora  



## 📹 Video de Demostración



## 📝 Descripción

Sistema solar interactivo renderizado con WGPU (WebGPU) en Rust. El proyecto presenta un sistema solar completo con planetas orbitando, shaders procedurales únicos para cada cuerpo celeste, una nave espacial que sigue a la cámara, y un sistema completo de navegación 3D.

## ✨ Características Implementadas

### Requerimientos Base ✅
- ✅ Sol renderizado con shader de plasma animado
- ✅ 7 planetas con órbitas circulares en el plano eclíptico
- ✅ Sistema orbital con rotación sobre eje propio
- ✅ Cámara 3D con movimiento completo

### Features Adicionales ⭐
- ✅ **Movimiento 3D de cámara** (40 pts) - Control completo en 3 ejes con WASD + QE
- ✅ **Nave espacial siguiendo cámara** (30 pts) - Modelo OBJ personalizado
- ✅ **Órbitas visibles** (20 pts) - Líneas que muestran las trayectorias
- ✅ **Instant warping animado** (20 pts) - Teletransporte suave entre planetas
- ✅ **Sistema de colisiones** (10 pts) - Previene atravesar planetas
- ✅ **Skybox mejorado** (10 pts) - 500+ estrellas distribuidas uniformemente

**Puntos totales:** 130+ puntos

## 🎮 Controles

### Movimiento de Cámara
- **W** - Avanzar
- **S** - Retroceder
- **A** - Mover izquierda
- **D** - Mover derecha
- **Q** - Bajar
- **E** - Subir
- **Mouse** (mantener click) - Rotar vista

### Navegación
- **1** - Warp al Sol
- **2** - Warp a Mercurio
- **3** - Warp a Venus
- **4** - Warp a la Tierra
- **5** - Warp a Marte
- **6** - Warp a Júpiter
- **7** - Warp a Saturno

### General
- **ESC** - Salir de la aplicación

## 🏗️ Arquitectura del Proyecto

```
src/
├── lib.rs                          # Biblioteca principal
├── main.rs                         # Punto de entrada
├── camera/
│   └── mod.rs                      # Cámara 3D con controles completos
├── scene/
│   ├── mod.rs
│   ├── celestial_body.rs           # Definición de planetas y lunas
│   ├── orbital_system.rs           # Sistema orbital completo
│   ├── spaceship.rs                # Nave espacial con carga OBJ
│   └── skybox.rs                   # Generación de estrellas
├── physics/
│   ├── mod.rs
│   └── collision.rs                # Detección de colisiones esféricas
├── input/
│   └── mod.rs                      # Sistema de entrada unificado
├── render/
│   ├── mod.rs                      # Renderizador WGPU
│   ├── shader.wgsl                 # Shaders procedurales
│   └── uniforms.rs                 # Estructuras de datos GPU
└── utils/
    ├── mod.rs
    └── geometry.rs                 # Generación de geometría
```

## 🎨 Shaders Procedurales

Cada cuerpo celeste tiene su propio shader proceduralúnico:

1. **Sol** - Plasma animado con manchas solares y corona pulsante
2. **Mercurio** - Planeta rocoso con cráteres
3. **Venus** - Planeta volcánico con lava
4. **Tierra** - Planeta gaseoso (shader azul)
5. **Luna** - Superficie helada con cráteres
6. **Marte** - Terreno rocoso rojo
7. **Júpiter** - Bandas atmosféricas con Gran Mancha Roja
8. **Saturno** - Planeta con anillos espectaculares

## 🚀 Compilación y Ejecución

### Requisitos
- Rust 1.70+
- GPU compatible con Vulkan, Metal, o DirectX 12

### Compilar
```bash
git clone [URL_REPOSITORIO]
cd sistema-solar-pablo-cabrera
cargo build --release
```

### Ejecutar
```bash
cargo run --release
```

## 📦 Dependencias

```toml
wgpu = "0.19"              # Renderizado GPU
winit = "0.29"             # Manejo de ventanas
nalgebra-glm = "0.18"      # Matemáticas 3D
bytemuck = "1.14"          # Conversión de datos GPU
rand = "0.8"               # Generación aleatoria
```

## 🎓 Conceptos Aplicados

### Gráficas por Computadora
- Pipeline de renderizado moderno (WGPU)
- Transformaciones 3D (matrices de vista y proyección)
- Shaders WGSL (vertex y fragment shaders)
- Iluminación básica (diffuse lighting)
- Texturas procedurales (noise functions, Voronoi)

### Física Básica
- Sistema orbital simplificado
- Colisiones esféricas
- Interpolación de movimiento (ease-in-out)

### Arquitectura de Software
- Separación de responsabilidades (módulos)
- ECS ligero (Entity Component System)
- Event-driven architecture (winit)

## 🐛 Problemas Conocidos

- El modelo de nave puede no cargar si la ruta no es correcta
- Performance puede variar según la GPU

## 🔮 Mejoras Futuras

- [ ] Texturas reales de planetas (NASA)
- [ ] Sistema de partículas para estelas de nave
- [ ] UI con información de planetas
- [ ] Más lunas para cada planeta
- [ ] Cálculo orbital real (Kepler)
- [ ] Modo VR

## 📚 Referencias

- [WGPU Tutorial](https://sotrh.github.io/learn-wgpu/)
- [WebGPU Shading Language Spec](https://www.w3.org/TR/WGSL/)
- [NASA Solar System](https://solarsystem.nasa.gov/)
- [Book of Shaders](https://thebookofshaders.com/)

## 📄 Licencia

Este proyecto fue creado con fines educativos para el curso de Gráficas por Computadora.

## 🙏 Agradecimientos

- Profesor del curso por las enseñanzas
- Comunidad de Rust por la documentación
- WGPU team por la excelente biblioteca

---

**Developed with ❤️ and Rust 🦀**
