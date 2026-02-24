use burn_onnx::ModelGen;
use std::path::Path;

fn main() {
    // Generate model code from PaddleOCR ONNX models.
    // Model weights are stored in .burnpack format and loaded at runtime.
    //
    // Note: ONNX model files must be present in the paddleocr-models/ directory.
    // Download them from the PaddleOCR model zoo before building:
    // - ppocrv4_mobile_det.onnx (text detection)
    // - en_ppocrv4_mobile_rec.onnx (text recognition)

    let models = [
        ("paddleocr-models/ppocrv4_mobile_det.onnx", "text detection"),
        ("paddleocr-models/en_ppocrv4_mobile_rec.onnx", "text recognition"),
    ];

    for (model_path, description) in models {
        if Path::new(model_path).exists() {
            println!("cargo:rerun-if-changed={}", model_path);
            ModelGen::new()
                .input(model_path)
                .out_dir("model/")
                .run_from_script();
        } else {
            println!(
                "cargo:warning=ONNX model not found: {} ({}). Skipping model generation.",
                model_path, description
            );
            println!(
                "cargo:warning=To enable OCR functionality, download PaddleOCR models to paddleocr-models/"
            );
        }
    }
}
