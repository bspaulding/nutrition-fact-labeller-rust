use burn_import::onnx::ModelGen;

fn main() {
    // Generate Burn code from ONNX models at compile time
    // This converts the ONNX models into native Burn code
    
    println!("cargo:rerun-if-changed=models/ppocrv4_mobile_det.onnx");
    println!("cargo:rerun-if-changed=models/en_ppocrv4_mobile_rec.onnx");
    
    // Generate detection model
    ModelGen::new()
        .input("models/ppocrv4_mobile_det.onnx")
        .out_dir("generated/")
        .run_from_script();
    
    // Generate recognition model  
    ModelGen::new()
        .input("models/en_ppocrv4_mobile_rec.onnx")
        .out_dir("generated/")
        .run_from_script();
}
