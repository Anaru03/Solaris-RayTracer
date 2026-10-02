# 🌌 SOLAR//TRACE

<p align="center">
  <b>Voxel Solar System rendered with CPU Ray Tracing</b>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-DEA584?style=for-the-badge&logo=rust&logoColor=black" />
  <img src="https://img.shields.io/badge/Ray%20Tracing-CPU-00bcd4?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Voxel-Diorama-6C63FF?style=for-the-badge" />
  <img src="https://img.shields.io/badge/status-en%20desarrollo-orange?style=for-the-badge" />
</p>

---

## ☀️ Sobre el proyecto

**SOLAR//TRACE** es un diorama interactivo del Sistema Solar desarrollado en **Rust** para el curso de **Gráficas por Computadora**.

La escena es renderizada mediante **ray tracing en CPU** y representa el Sol, los ocho planetas y la Luna. Los planetas están construidos con cubos para conservar una estética **voxel**, mientras orbitan dinámicamente alrededor del Sol.

El usuario puede recorrer el sistema, rotar la cámara, hacer zoom y seleccionar individualmente cada cuerpo celeste.

---

## ✨ Características

- ☀️ Sol y los 8 planetas del Sistema Solar
- 🌙 Luna orbitando alrededor de la Tierra
- 🪐 Movimiento orbital en tiempo real
- 🧊 Planetas construidos con cubos
- 🔭 Cámara orbital interactiva
- 🔎 Zoom
- 🖱️ Selector interactivo de cuerpos celestes
- 🌀 Órbitas visibles
- 💡 Iluminación mediante ray tracing
- 🎨 Texturas BMP
- 🧱 Sistema de materiales
- ⚡ Renderizado optimizado en CPU

---

## 🎮 Controles

| Control | Acción |
|:---:|---|
| `Click` | Seleccionar desde el menú |
| `1` | Sol |
| `2` | Mercurio |
| `3` | Venus |
| `4` | Tierra |
| `5` | Marte |
| `6` | Júpiter |
| `7` | Saturno |
| `8` | Urano |
| `9` | Neptuno |
| `0` | Vista completa |
| `← → ↑ ↓` | Rotar cámara |
| `W / S` | Acercar / alejar |
| `Esc` | Salir |

---


## 🚀 Ejecución

```bash
cargo run --release
```

> Se recomienda utilizar `--release` debido a que el ray tracing se ejecuta en CPU.

---

## 📹 Demo

<!-- Agregar aquí el video final del diorama -->

**Video:** [Ver video en YouTube](https://youtu.be/0MG1Mqkkasg?si=vrEo9Ime_c2HsV2z)

---

## 🛠️ Tecnologías

**Rust · Ray Tracing · Voxel Rendering · BMP · minifb**

---

<p align="center">
  <b>SOLAR//TRACE</b><br/>
  <i>Explore the system. One voxel at a time.</i>
</p>
