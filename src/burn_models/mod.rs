// Burn-based OCR models
// These models are generated at compile-time from ONNX files

pub mod ppocrv4_mobile_det {
    include!(concat!(env!("OUT_DIR"), "/generated/ppocrv4_mobile_det.rs"));
}

pub mod en_ppocrv4_mobile_rec {
    include!(concat!(env!("OUT_DIR"), "/generated/en_ppocrv4_mobile_rec.rs"));
}
