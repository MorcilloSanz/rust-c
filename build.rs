fn main() {
    cc::Build::new()
        .file("lib/add.c")
        .file("lib/subtract.c")
        .include("lib")  // Incluir las cabeceras si es necesario
        .compile("libmath.a");  // Nombre del archivo de salida (estático)
}