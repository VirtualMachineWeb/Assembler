proc start:
cpl_u8 0x1
push_u8 0x0
cmp_u8
jmp_true &finish
push_u64 &poploop
cpl_u8 0x9
jmp &console.printc
poploop:
pop_u8
jmp &this.start
finish:
pop_u8
push_u8 0xA
jmp &console.printc
end proc