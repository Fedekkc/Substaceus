# 🧠 Substaceus

_Uncover what lies beneath the surface._

**Substaceus** es una herramienta escrita en Rust para descubrir subdominios de forma rápida, sencilla y elegante. Pensada para ser usada desde línea de comandos, soporta wordlists personalizadas y ejecución multihilo para acelerar búsquedas masivas.

---

## 🚀 Características

- 🔍 Descubrimiento de subdominios por diccionario.
- ⚡ Resolución DNS paralela con múltiples hilos. [TODO]
- 🌈 Salida colorida y clara.
- 🧰 Wordlist personalizable.
- 🧵 Control de concurrencia configurable por parámetro. [TODO]


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
.\target\release\subdomain-finder <dominio> [wordlist] [hilos]
```

#### 🐧 En Linux/macOS:

```bash
./target/release/subdomain-finder <dominio> [wordlist] [hilos]
```

---

### 📌 Ejemplos

```bash
# Usar wordlist por defecto (subdomains.txt) y 4 hilos
./target/release/subdomain-finder google.com

# Usar wordlist personalizada y 10 hilos
./target/release/subdomain-finder ejemplo.com custom.txt 10
```

---

> Asegurate de tener una wordlist válida en el mismo directorio o especificar la ruta correcta.

## 📦 Requisitos

- Rust

- Wordlist de subdominios (por defecto usa subdomains.txt)

## 💡 Ideas a futuro
- Soporte asincrónico con Tokio.

- Resolución recursiva de sub-subdominios.

- Exportar resultados a JSON/CSV.

- Detección de wildcard DNS.

- Integración con servicios online (Shodan, Censys, etc.)


_Creado con❤️ por @Fedekkc_
