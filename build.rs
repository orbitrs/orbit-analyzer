// This build script ensures that advapi32.lib is properly linked on Windows
// which is needed by the Skia ICU library for Registry API functions

fn main() {
    // Only add Windows-specific linking on Windows MSVC target
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    {
        // Link advapi32.lib which contains the Windows Registry API
        println!("cargo:rustc-link-lib=dylib=advapi32");
        
        // Ensure the linker can find the library
        println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
    }
}
