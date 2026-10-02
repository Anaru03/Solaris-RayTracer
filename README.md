# 🌌 SOLAR//TRACE

<div align="center">

### 🪐 Voxel Solar System · CPU Ray Tracing · Rust

**Un Sistema Solar interactivo construido desde cero mediante Ray Tracing.**

![Rust](https://img.shields.io/badge/Rust-DEA584?style=for-the-badge&logo=rust&logoColor=black)
![Ray Tracing](https://img.shields.io/badge/Ray%20Tracing-CPU-00BCD4?style=for-the-badge)
![Voxel](https://img.shields.io/badge/Style-Voxel-7C4DFF?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Completed-2EA44F?style=for-the-badge)

</div>

---

## 🚀 El proyecto

**SOLAR//TRACE** es un diorama interactivo del Sistema Solar desarrollado en **Rust** para el curso de **Gráficas por Computadora**.

La escena utiliza **Ray Tracing en CPU** para representar el Sol, los ocho planetas y la Luna dentro de un entorno espacial dinámico.

Los planetas combinan **geometría voxel, texturas individuales, materiales e iluminación**, mientras orbitan alrededor del Sol en tiempo real.

> Explora el sistema, selecciona un planeta y obsérvalo desde diferentes perspectivas.

---

## 🎥 Demo

<div align="center">

### ▶️ [Ver SOLAR//TRACE en YouTube](https://youtu.be/Nd9oQ3XtJxs)

[![Ver video](https://img.shields.io/badge/YouTube-Ver%20Demo-FF0000?style=for-the-badge&logo=youtube&logoColor=white)](https://youtu.be/Nd9oQ3XtJxs)

</div>

En el video se muestran el movimiento orbital, navegación de cámara, selección de cuerpos celestes, texturas, materiales, reflexión, refracción, skybox y geometría voxel.

---

## ✨ Características

| | Implementación |
|---|---|
| ☀️ | Sol texturizado |
| 🪐 | 8 planetas con movimiento orbital |
| 🌙 | Luna orbitando alrededor de la Tierra |
| 🧊 | Geometría voxel construida con cubos |
| 💫 | Anillos voxel 3D para Saturno |
| 🎨 | Texturas individuales BMP |
| 💡 | Iluminación difusa y especular |
| 🪞 | Reflexión mediante rayos secundarios |
| 💎 | Transparencia y refracción |
| 🌌 | Skybox espacial procedural |
| 🔭 | Cámara orbital interactiva |
| 🔎 | Zoom |
| 🖱️ | Selector interactivo de planetas |
| ⚡ | Culling mediante volúmenes envolventes |

---

## 🪐 Explora el Sistema Solar

```text
                         ☀ SUN

          MER      VEN      EAR      MAR

               JUP      SAT 🪐

                    URA      NEP
```

La escena incluye:

**☀ Sol · Mercurio · Venus · 🌎 Tierra · Marte · Júpiter · 🪐 Saturno · Urano · Neptuno**

Cada planeta posee su propia:

- Textura
- Escala
- Órbita
- Velocidad orbital
- Configuración de material

Saturno incluye además **anillos construidos mediante cubos**, manteniendo la estética voxel del proyecto.

---

## 🎮 Controles

| Control | Acción |
|:---:|---|
| 🖱️ `Click` | Seleccionar un cuerpo desde el menú |
| `1` | ☀️ Sol |
| `2` | Mercurio |
| `3` | Venus |
| `4` | 🌎 Tierra |
| `5` | Marte |
| `6` | Júpiter |
| `7` | 🪐 Saturno |
| `8` | Urano |
| `9` | Neptuno |
| `0` | 🌌 Vista completa del sistema |
| `← →` | Rotar horizontalmente |
| `↑ ↓` | Rotar verticalmente |
| `W` | Acercar |
| `S` | Alejar |
| `Esc` | Salir |

---

## 🎨 Ray Tracing

Cada píxel de la escena comienza con un rayo generado desde la cámara.

```text
                   CAMERA
                      │
                      ▼
                 PRIMARY RAY
                      │
              ┌───────┴───────┐
              │               │
            HIT             NO HIT
              │               │
              ▼               ▼
        Material + UV       SKYBOX
              │
      ┌───────┼────────┐
      ▼       ▼        ▼
   DIFFUSE  SPECULAR  TEXTURE
      │
      ├──────────────► REFLECTION
      │
      └──────────────► REFRACTION
              │
              ▼
          FINAL COLOR
```

El raytracer calcula la intersección más cercana y aplica las propiedades correspondientes del material.

---

## 💎 Materiales

Los cuerpos celestes utilizan materiales con diferentes propiedades:

```rust
Material {
    albedo,
    specular,
    transparency,
    reflectivity,
    refractive_index,
}
```

Esto permite que los planetas reaccionen de manera diferente ante la iluminación y los rayos secundarios.

El sistema implementa:

**Diffuse · Specular · Reflection · Transparency · Refraction**

---

## 🖼️ Texturas

Cada cuerpo principal posee una textura individual:

```text
assets/
└── textures/
    ├── sun.bmp
    ├── mercury.bmp
    ├── venus.bmp
    ├── earth.bmp
    ├── mars.bmp
    ├── jupiter.bmp
    ├── saturn.bmp
    ├── uranus.bmp
    └── neptune.bmp
```

El proyecto incluye un lector propio para imágenes **BMP de 24 y 32 bits**.

Las texturas también son utilizadas en el selector visual de la interfaz.

---

## ⚡ Optimización

El Ray Tracing se ejecuta completamente en CPU, por lo que la escena utiliza varias optimizaciones:

- **Bounding Sphere Culling**
- Rayos secundarios limitados
- Geometría voxel controlada
- Resolución interna optimizada
- Texturas cargadas una sola vez
- Renderizado en modo `release`

Antes de comprobar todos los cubos de un planeta, el raytracer verifica si el rayo intersecta su volumen envolvente.

```text
RAY
 │
 ▼
BOUNDING SPHERE?
 │
 ├── NO ──────► SKIP
 │
 └── YES
       │
       ▼
   VOXEL CUBES
```

---

## 🧱 Estructura

```text
Trace404-TheLastDebug/
│
├── assets/
│   └── textures/
│
├── src/
│   ├── main.rs
│   ├── camera.rs
│   ├── cube.rs
│   ├── framebuffer.rs
│   ├── material.rs
│   ├── planet.rs
│   ├── ray.rs
│   ├── sphere.rs
│   ├── texture.rs
│   └── vector.rs
│
├── Cargo.toml
└── README.md
```

---

## 🚀 Ejecutar

Clona el repositorio y entra al proyecto:

```bash
git clone <URL-DEL-REPOSITORIO>
cd Trace404-TheLastDebug
```

Ejecuta la versión optimizada:

```bash
cargo run --release
```

> `--release` es recomendado debido a que el Ray Tracing se realiza en CPU.

---

## 🛠️ Tecnologías

<div align="center">

**Rust** · **Ray Tracing** · **Voxel Rendering** · **minifb** · **BMP**

</div>

---

<div align="center">

## ☀️ SOLAR//TRACE

**Explore the system. One voxel at a time.**

`SUN` · `MER` · `VEN` · `EAR` · `MAR` · `JUP` · `SAT` · `URA` · `NEP`

</div>