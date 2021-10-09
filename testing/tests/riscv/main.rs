use dynasmrt::dynasm;

#[test]
fn riscv() {
     let mut ops = dynasmrt::SimpleAssembler::new();
     dynasm!(ops
             ; .arch riscv64
             ; li a0, 1
     );
     let buf = ops.finalize();
     let hex: Vec<String> = buf.iter().map(|x| format!("0x{:02X}", *x)).collect();
     let hex: String = hex.join(", ");
     assert_eq!(hex, "0x00, 0x10, 0x05, 0x13", "li a0, 1");
}
