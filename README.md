# **Holomorphic-Tinkering** (Deutsch)

Dieses Projekt kombiniert Rust und WebAssembly, um Bildverarbeitungsalgorithmen im Browser auszuführen. Es verwendet eine Rust-Bibliothek, die in WebAssembly kompiliert wird, um Bilder zu transformieren und zu manipulieren. Die Frontend-Anwendung (HINWEIS: nicht komplett implementiert, erfordert Nachbesserungen) ist in React geschrieben und ermöglicht es Benutzern, Bilder hochzuladen und zu transformieren oder die Transformationen in Echtzeit zu sehen via Webcam.

Das Projekt nutzt holomorphe Funktionen, um komplexe Bildtransformationen durchzuführen. Holomorphe Funktionen sind in der komplexen Analysis von Bedeutung und ermöglichen es, glatte und kontinuierliche Transformationen auf Bildern anzuwenden, was zu beeindruckenden visuellen Effekten führt.

---

## **Voraussetzungen**

### Allgemeine Anforderungen
- **Rust**: Rust muss installiert sein, um dieses Projekt auszuführen.
    - Rust über den offiziellen [Installationsleitfaden](https://www.rust-lang.org/tools/install) installieren.
    - Für Linux:
      ```bash
      sudo apt install cargo
      ```
    - Für Windows:  
      Den Rust-Installer von der [offiziellen Website](https://www.rust-lang.org/tools/install) herunterladen und ausführen. Stellen Sie sicher, dass notwendige Tools (`cargo`, `rustc`) zur PATH-Umgebung hinzugefügt werden.

- **Node.js**: Zum Ausführen der Frontend-Anwendung wird eine Node.js-Laufzeitumgebung benötigt. Die einfachste Möglichkeit, Node.js-Versionen zu verwalten, ist die Verwendung von **nvm**:
    - Für Linux/Mac:  
      Dem [nvm-Installationsleitfaden](https://github.com/nvm-sh/nvm) folgen.
    - Für Windows:  
      [nvm-windows](https://github.com/coreybutler/nvm-windows/releases) installieren und zur PATH-Umgebung hinzufügen. Anschließend Node.js installieren:
      ```bash
      nvm install Iron
      nvm use Iron
      ```

- **pnpm** (optional): Ein schnellerer Node.js-Paketmanager. Installation von **pnpm**:
    - Universelle Installation:
      ```bash
      npm install --global pnpm
      ```
    - Abhängigkeiten installieren und das Frontend ausführen (HINWEIS: nicht komplett implementiert, erfordert Nachbesserungen, bis dahin diese folgenden (a bis c) Schritte überspringen:
        1. Navigieren Sie zum Projektverzeichnis.
        2. Führen Sie folgende Befehle aus:
        ```bash
        pnpm -F react-frontend install
        pnpm run frontend
        ```
        3. Fehlerbehebung:
        ```bash
        cargo install wasm-pack
        wasm-pack build --target web --out-dir pkg
        ```

- **OpenCV**: Erforderlich für Webcam-Funktionalitäten.
    - Für Linux:
      ```bash
      sudo apt update
      sudo apt install libopencv-dev pkg-config
      ```
      (NUR) Falls das zu Problemen führt (z.B.Pakete fehlen), kann OpenCV aus dem Quellcode installiert werden:
      ```bash
      sudo apt install build-essential cmake git pkg-config libgtk-3-dev libavcodec-dev \
      libavformat-dev libswscale-dev libv4l-dev libxvidcore-dev libx264-dev libjpeg-dev \
      libpng-dev libtiff-dev gfortran openexr libatlas-base-dev python3-dev python3-numpy \
      libtbb2 libtbb-dev libdc1394-22-dev
      git clone https://github.com/opencv/opencv.git
      git clone https://github.com/opencv/opencv_contrib.git
      cd opencv && mkdir build && cd build
      cmake -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX=/usr/local \
      -D OPENCV_EXTRA_MODULES_PATH=../../opencv_contrib/modules ..
      make -j$(nproc)
      sudo make install
      sudo ldconfig
      ```
    - Für Windows:
        1. OpenCV von der [offiziellen Website](https://opencv.org/releases/) herunterladen und installieren.
        2. Die Umgebungsvariable `OpenCV_DIR` auf den OpenCV-Installationsordner setzen (z. B. `C:\opencv\build`).

---

## **Ausführen des Programms**

### 1. Repository klonen
```bash
git clone https://github.com/alexinatra2/holo.git
cd holo
```

### 2. Programm ausführen
```bash
cargo run "[FUNCTION]"
```

- **Webcam**: Um die Webcam und Echtzeittransformation zu nutzen:
  ```bash
  cargo run "[FUNCTION]"
  ```
- **Bildtransformation**: Um ein Bild zu transformieren (Pfad relativ vom Projekt-root siehe Beispiele):
  ```bash
  cargo run "[FUNCTION]" -i [Pfad]
  ```
  Das transformierte Bild wird im Verzeichnis `./images/output/` gespeichert. Der Dateiname setzt sich aus dem Namen der Eingabedatei und einem Zeitstempel zusammen.

---

## **Beispiele**
Alle möglichen Parameter sowohl für Webcam- als auch für Bildertransformation verfügbar. (siehe Verfügbare Optionen)

### Webcam Transformation
1. Polynomische Funktion:
   ```bash
   cargo run "z + 2*z^2 + 3*z^3"
   ```
2. Gebrochenrationale Funktion:
   ```bash
   cargo run "(z + 2*z^2 + 3) / (1 + z^3)"
   ```

### Bild Transformation
1. Benutzerdefinierte Dimensionen:
   ```bash
   cargo run "z + 2*z^2" -i ./images/input/test.jpg -d 800,600
   ```
2. Vordefinierte Auflösung:
   ```bash
   cargo run "z^2 / (1 + z)" -i ./images/input/test.jpg -r hd
   ```

---

## **Verfügbare Optionen**
Mit `--help` weitere Optionen anzeigen:
```bash
cargo run -- --help
```

### Optionen
```
Usage: holo [OPTIONS] <FUNCTION>

Arguments:
  <FUNCTION>  Function to apply to the file contents

Options:
  -i, --image <IMAGE_FILENAME>   Path to the file to process
  -r, --resolution <RESOLUTION>  Resolution preset, overriding custom dimensions if specified [possible values: hd, full-hd, uhd, qhd, wqhd, four-k, eight-k, sd, retina, svga, xga, wxga, hd-ready, wvga, qvga, cga]
  -d, --dimensions <DIMENSIONS>  Custom dimensions in the format width,height
  -h, --help                     Print help
  -V, --version                  Print version
```

---

# **Holomorphic-Tinkering** (English)

This project combines Rust and WebAssembly to execute image processing algorithms in the browser. It uses a Rust library compiled to WebAssembly to transform and manipulate images. The frontend application ((NOTE: not fully implemented, requires improvement)) is written in React and allows users to upload images and to transform or see the transformations in real-time via webcam.

The project utilizes holomorphic functions to perform complex image transformations. Holomorphic functions are significant in complex analysis and allow for smooth and continuous transformations on images, resulting in impressive visual effects.

---

## **Prerequisites**
... *(The English prerequisites section is the same as above in German)* ...

---

## **How to Run**

### 1. Clone the repository
```bash
git clone https://github.com/alexinatra2/holo.git
cd holo
```

### 2. Run the program
```bash
cargo run "[FUNCTION]"
```

- **Webcam**:
  ```bash
  cargo run "[FUNCTION]"
  ```
- **Image Transformation** (Path relative to project-root, e.q. examples:
  ```bash
  cargo run "[FUNCTION]" -i [path]
  ```
  The transformed image is saved in the directory `./images/output/`.

---

## **Examples**
### Webcam Transformation
1. Polynomial function:
   ```bash
   cargo run "z + 2*z^2 + 3*z^3"
   ```
2. Rational function:
   ```bash
   cargo run "(z + 2*z^2 + 3) / (1 + z^3)"
   ```

### Image Transformation
1. Custom dimensions:
   ```bash
   cargo run "z + 2*z^2" -i ./images/input/test.jpg -d 800,600
   ```
2. Predefined resolution:
   ```bash
   cargo run "z^2 / (1 + z)" -i ./images/input/test.jpg -r hd
   ```

---

## **Options**
Use `--help` for more information:
```bash
cargo run -- --help
```

### Options
```
Usage: holo [OPTIONS] <FUNCTION>

Arguments:
  <FUNCTION>  Function to apply to the file contents

Options:
  -i, --image <IMAGE_FILENAME>   Path to the file to process
  -r, --resolution <RESOLUTION>  Resolution preset, overriding custom dimensions if specified [possible values: hd, full-hd, uhd, qhd, wqhd, four-k, eight-k, sd, retina, svga, xga, wxga, hd-ready, wvga, qvga, cga]
  -d, --dimensions <DIMENSIONS>  Custom dimensions in the format width,height
  -h, --help                     Print help
  -V, --version                  Print version
