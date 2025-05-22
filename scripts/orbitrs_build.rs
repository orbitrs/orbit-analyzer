// This build script ensures all required libraries are properly linked
// It's particularly important for Windows where advapi32.lib is needed for the Skia ICU library

fn main() {
    // Windows-specific linking for MSVC target
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    {
        // Link advapi32.lib which contains the Windows Registry API
        println!("cargo:rustc-link-lib=dylib=advapi32");
        
        // Ensure the linker can find the library
        println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
    }
    
    // Add any future platform-specific dependencies here
}
