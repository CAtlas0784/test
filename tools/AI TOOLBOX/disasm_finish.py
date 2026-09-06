import pefile, capstone, json

dll_path = r"E:\beta hsr\StarRail_4.5.52_OS\GameAssembly.dll"
pe = pefile.PE(dll_path, fast_load=True)

with open(r"E:\beta hsr\StarRail_4.5.52_OS\DUMP\methods.json", "r", encoding="utf-8", errors="ignore") as f:
    methods = json.load(f)

addr_to_name = {int(v, 16): k for k, v in methods.items() if v.startswith("0x")}

md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)

def disasm_func(rva, max_bytes=200):
    offset = pe.get_offset_from_rva(rva)
    with open(dll_path, "rb") as f:
        f.seek(offset)
        code = f.read(max_bytes)

    lines = []
    for insn in md.disasm(code, rva):
        line = f"0x{insn.address:x}: {insn.mnemonic} {insn.op_str}"
        if insn.mnemonic.startswith("call"):
            try:
                target = int(insn.op_str, 16)
                if target in addr_to_name:
                    line += f"  ; {addr_to_name[target]}"
            except:
                pass
        lines.append(line)
        if insn.mnemonic == "ret" and len(lines) > 5:
            break
    return lines

print("--- GetMaxFinishLevel35 (0x19CFA930) ---")
for l in disasm_func(0x19CFA930, 250):
    print(l)
