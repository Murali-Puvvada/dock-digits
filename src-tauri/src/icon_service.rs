use objc2_app_kit::{NSBitmapImageFileType, NSBitmapImageRep, NSWorkspace};
use objc2_foundation::{NSDictionary, NSString};

use sha1::{Digest, Sha1};
use std::fs;
use std::path::PathBuf;

pub fn get_icon_for_app(path: &str) -> Option<String> {
    let cache_path = icon_cache_path(path);

    if cache_path.exists() {
        if let Ok(bytes) = fs::read(&cache_path) {
            use base64::{engine::general_purpose, Engine as _};
            let b64 = general_purpose::STANDARD.encode(bytes);
            return Some(format!("data:image/png;base64,{}", b64));
        }
    }

    // 1. Get the workspace
    let workspace = NSWorkspace::sharedWorkspace();

    // 2. Convert path to NSString
    let ns_path = NSString::from_str(path);

    // 3. iconForFile returns Retained<NSImage>, not Option<Retained<NSImage>>.
    let icon = workspace.iconForFile(&ns_path);

    // 4. These usually DO return Option, so keep the '?' here
    let tiff_data = icon.TIFFRepresentation()?;
    let bitmap = NSBitmapImageRep::imageRepWithData(&tiff_data)?;

    // 6. Convert to PNG
    // Note: In 0.2.2, the enum variant is often just 'PNG' or 'NSPNG'
    let png_data = unsafe {
        bitmap.representationUsingType_properties(NSBitmapImageFileType::PNG, &NSDictionary::new())
    }?;

    // 7. Get the bytes
    let bytes = png_data.to_vec();
    fs::create_dir_all(cache_path.parent()?).ok()?;
    fs::write(&cache_path, &bytes).ok()?;

    use base64::{engine::general_purpose, Engine as _};
    let b64 = general_purpose::STANDARD.encode(bytes);
    Some(format!("data:image/png;base64,{}", b64))
}

fn icon_cache_path(app_path: &str) -> PathBuf {
    let mut hasher = Sha1::new();
    hasher.update(app_path);

    let hash = format!("{:x}", hasher.finalize());

    let mut path = dirs::cache_dir().unwrap();

    path.push("dock-digits");
    path.push("icons");
    path.push(format!("{}.png", hash));

    path
}
