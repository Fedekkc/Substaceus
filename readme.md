# 🧠 Substaceus

![GitHub stars](https://img.shields.io/github/stars/Fedekkc/Substaceus?style=social)
![GitHub license](https://img.shields.io/github/license/Fedekkc/Substaceus)
![GitHub last commit](https://img.shields.io/github/last-commit/Fedekkc/Substaceus)
![Rust](https://img.shields.io/badge/made%20with-rust-orange?logo=rust)


_Uncover what lies beneath the surface._

**Substaceus** es una herramienta escrita en Rust para descubrir subdominios de forma rápida, sencilla y elegante. Pensada para ser usada desde línea de comandos, soporta wordlists personalizadas y ejecución multihilo para acelerar búsquedas masivas.

---

## 🚀 Características

- 🔍 Descubrimiento de subdominios por diccionario.
- ⚡ Resolución DNS paralela con múltiples hilos. 
- 🌈 Salida colorida y clara.
- 🧰 Wordlist personalizable.
- 📄 Export en JSON/TXT
- 🧵 Control de concurrencia.


---

## ⚙️ Uso

### 📦 Compilación

Primero compilá el binario optimizado:

```bash
cargo build --release
```

Esto generará el ejecutable en `target/release/subdomain-finder`.

---

### 💻 Ejecución

#### 🪟 En Windows PowerShell:

```powershell
.\target\release\subdomain-finder <dominio> [wordlist] [JSON/TXT]
```

#### 🐧 En Linux/macOS:

```bash
./target/release/subdomain-finder <dominio> [wordlist] [JSON/TXT]
```

---

### 📌 Ejemplos

```bash
# Usar wordlist por defecto (subdomains.txt) y no exportar
./target/release/subdomain-finder google.com

# Usar wordlist personalizada y exportar
./target/release/subdomain-finder google.com custom_wordlist.txt  
```

---

> Asegurate de tener una wordlist válida en el mismo directorio o especificar la ruta correcta.

## 📦 Requisitos

- Rust

- Wordlist de subdominios (por defecto usa subdomains.txt)

## 💡 Ideas a futuro

- Resolución recursiva de sub-subdominios.

- Detección de wildcard DNS.

- Integración con servicios online (Shodan, Censys, etc.)


_Creado con❤️ por @Fedekkc_
