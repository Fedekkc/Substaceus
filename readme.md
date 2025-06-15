# 🧠 Substaceus

_Uncover what lies beneath the surface._

**Substaceus** es una herramienta escrita en Rust para descubrir subdominios de forma rápida, sencilla y elegante. Pensada para ser usada desde línea de comandos, soporta wordlists personalizadas y ejecución multihilo para acelerar búsquedas masivas.

---

## 🚀 Características

- 🔍 Descubrimiento de subdominios por diccionario.
- ⚡ Resolución DNS paralela con múltiples hilos.
- 🌈 Salida colorida y clara.
- 🧰 Wordlist personalizable.
- 🧵 Control de concurrencia configurable por parámetro.

---

## 🛠️ Instalación

```batch
git clone https://github.com/Fedekkc/Substaceus.git
cd Substaceus
cargo build --release

---

## ⚙️ Uso
./subdomine-finder <dominio> [wordlist] [hilos]
# Usar con defaults
./subdomine-finder ejemplo.com

# Usar wordlist personalizada y 10 hilos
./subdomine-finder ejemplo.com lista.txt 10
📦 Requisitos
Rust

Wordlist de subdominios (por defecto usa subdomains.txt)

💡 Ideas a futuro
Soporte asincrónico con Tokio.

Resolución recursiva de sub-subdominios.

Exportar resultados a JSON/CSV.

Detección de wildcard DNS.

Integración con servicios online (Shodan, Censys, etc.)

👤 Autor
Creado con 💻 por @Fedekkc

-- “El conocimiento oculto no siempre está lejos, a veces solo está debajo de un prefijo.”
