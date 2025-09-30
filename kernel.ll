
; -------------------------
; Globals
; -------------------------
@vga = global i16* inttoptr (i32 753664 to i16*)
    

; -------------------------
; Kernel entry
; -------------------------
define void @main() {
entry:
    ; -------------------------
    ; clear_screen() → fill VGA with spaces (0x0F20)
    ; -------------------------
    %vga_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 0
    %i = alloca i32
    store i32 0, i32* %i
    ; -------------------------
    ; typed VGA writes
    ; -------------------------
    ; vga[0].value = (0x0F << 8) | 'H'
    %vga0_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 0
    store volatile i16 3928, i16* %vga0_ptr  ; 0x0F << 8 | 0x48 ('H')

    ; vga[1].value = (0x0F << 8) | 'i'
    %vga1_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 1
    store volatile i16 3945, i16* %vga1_ptr  ; 0x0F << 8 | 0x69 ('i')

    ; (~vga[10]).write16((0x0F << 8) | '!')
    %vga10_ptr = getelementptr [2000 x i16], [2000 x i16]* @vga, i32 0, i32 10
    store volatile i16 3873, i16* %vga10_ptr  ; 0x0F << 8 | 0x21 ('!')

    call void asm sideeffect "hlt", ""()

    ret void
}
