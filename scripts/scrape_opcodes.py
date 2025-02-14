'''
This script is used to scrape opcodes.rs for opcodes that belong to a specific addressing mode using their names
'''

# Dictionary: each instruction -> category (as strings)
category = {
    # Load
    "LDA": "LD",
    "LDX": "LD",
    "LDY": "LD",

    # Store
    "STA": "ST",
    "STX": "ST",
    "STY": "ST",

    # Register Transfers
    "TAX": "TX",
    "TAY": "TX",
    "TXA": "TX",
    "TYA": "TX",

    # Stack
    "TSX": "SK",
    "TXS": "SK",
    "PHA": "SK",
    "PHP": "SK",
    "PLA": "SK",
    "PLP": "SK",

    # Logical
    "AND": "LO",
    "EOR": "LO",
    "ORA": "LO",
    "BIT": "LO",

    # Arithmetic
    "ADC": "AR",
    "SBC": "AR",
    "CMP": "AR",
    "CPX": "AR",
    "CPY": "AR",

    # Increments/Decrements
    "INC": "ID",
    "INX": "ID",
    "INY": "ID",
    "DEC": "ID",
    "DEX": "ID",
    "DEY": "ID",

    # Shifts
    "ASL": "SH",
    "LSR": "SH",
    "ROL": "SH",
    "ROR": "SH",

    # Jumps/Calls
    "JMP": "JP",
    "JSR": "JP",
    "RTS": "JP",

    # Branches
    "BCC": "BR",
    "BCS": "BR",
    "BEQ": "BR",
    "BMI": "BR",
    "BNE": "BR",
    "BPL": "BR",
    "BVC": "BR",
    "BVS": "BR",

    # Status Flags
    "CLC": "SF",
    "CLD": "SF",
    "CLI": "SF",
    "CLV": "SF",
    "SEC": "SF",
    "SED": "SF",
    "SEI": "SF",

    # System Functions (Syscalls)
    "BRK": "SY",
    "NOP": "SY",
    "RTI": "SY"
}


def get_ops(addr_mode: str):
    f = open("../src/emulator/opcodes.rs").read().split('\n')
    for i,v in enumerate(f):
        try:
            if "Opcode" in v and addr_mode in v:
                to_print = v.split('=>')[1].split(',')[0].strip()
                print(to_print)
        except:
            continue

def parse_docs():
    '''
    Parse the document to build a static array for rust 
    '''
    
    opcode_str = 'const array: [Opcode; 256] = ['
    addr_mode_str = 'const array: [AddrMode; 256] = ['
    cycle_str = 'const array: [u8; 256] = ['
    optype_str = 'const array: [Inst; 256] = ['

    f = open('./opcode_docs.txt', 'r').read().split('\n')
    i = 0
    chunks = []
    new_chunk = []
    while i < len(f):
        v = f[i]
        if len(v) == 3:
            chunks.append(new_chunk)
            new_chunk = [v]
        else:
            new_chunk.append(v)
        i+=1
    chunks = chunks[1:]

    d = {
        'implied': 'IMP',
        'immediate': 'IMM',
        'accumulator': 'ACC',
        'zeropage	': 'ZPG',
        'zeropage,X': 'ZPX',
        'zeropage,Y': 'ZPY',
        'relative': 'REL',
        'absolute	': 'ABS',
        'absolute,X': 'ABX',
        'absolute,Y': 'ABY',
        'indirect	': 'IND',
        '(indirect,': 'IDX',
        '(indirect)': 'INX'
    }

    addr_mode_arr = [None] * 256
    cycle_arr = [None] * 256
    name_arr = [None] * 256 # format is opname + "_" + addrmode
    optype_arr = [None] * 256
    flag_arr = [None] * 256
    flag_str = "let flag_arr: [u8, 256] = ["

    for i,v in enumerate(chunks):
        skip_flag = 0
        to_append = 0
        for j,w in enumerate(v):
            if j == 0:
                inst_name = w
            addr_mode = None
            if '-	' in w: 
            # N	Z	C	I	D	V
            # +	+	-	-	-	-
                fl = w.split("	")
                if fl[0] == "+": # n
                    to_append |= (1<<7)
                if fl[1] == "+": # z
                    to_append |= (1<<1)
                if fl[2] == "+": # c
                    to_append |= 1
                if fl[3] == "+": # i
                    to_append |= (1<<2)
                if fl[4] == "+":
                    to_append |= (1<<3)
                if fl[5] == "+":
                    to_append |= (1<<6)

                print(fl)
                # input()
                skip_flag = 1
                continue
            if 'addressing' in w: continue
            if skip_flag == 0: continue


            # grabbing the addr mode
            for k in d.keys():
                if k in w: 
                    addr_mode = d[k]
                    break
            
            # grabbing the opcode
            opcode = w.split("	")[2]
            cycle = w.split("	")[4].strip().replace("*", "")
            
            print(f"{inst_name}: {addr_mode} with opcode: {opcode} cycle: {cycle}")
            pos = int(opcode, 16)
            flag_arr[pos] = to_append
            addr_mode_arr[pos] = 'AddrMode::' + addr_mode            
            cycle_arr[pos] = cycle
            name_arr[pos] = "Opcode::" + inst_name + "_" + addr_mode
            optype_arr[pos] = 'Inst::' + inst_name


    for i in range(256):
        if name_arr[i] == None:
            name_arr[i] = 'Opcode::ILL_OP' 
            addr_mode_arr[i] = 'AddrMode::ILL'
            cycle_arr[i] = 0
            optype_arr[i] = 'Inst::IL'
            flag_arr[i] = 0
        
        opcode_str += str(name_arr[i]) + ','
        addr_mode_str += str(addr_mode_arr[i]) + ','
        cycle_str += str(cycle_arr[i]) + ','
        optype_str += str(optype_arr[i]) + ','
        flag_str += f"{str(flag_arr[i])},"
        # cycle_str = 'const array: [u8; 256] = ['
        # optype_str = 'const array: [OpType; 256] = ['

        # print(addr_mode_arr[i] )
        # print(cycle_arr[i])
        # print(f"{name_arr[i][8:]} = {hex(i)},")
        # print(f"{hex(i)} => {name_arr[i]},")
        # print(optype_arr[i])

    opcode_str += ']'
    addr_mode_str += ']'
    cycle_str += ']'
    optype_str += ']'
    flag_str += "]"

    # print(opcode_str)
    # print(addr_mode_str)
    # print(cycle_str)
    # print(optype_str)
    print(flag_str)



               
def main():
    # get_ops('IMM')
    # get_ops('ABY')
    parse_docs()

if __name__ == '__main__':
    main()
