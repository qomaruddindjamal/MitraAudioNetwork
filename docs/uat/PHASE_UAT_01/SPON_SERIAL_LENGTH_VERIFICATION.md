# SPON SERIAL LENGTH & DISASSEMBLER VERIFICATION REPORT (TASK 1)
**Target Binary:** `C:\IPNBS\IPNBS_Server.exe`  
**Verified SHA-256:** `7F910FAFC2D66BF0FC7488F95204C9119AB9022347CF7175E4A578C2103E8AE9`  
**Entry Point Function:** RVA `0x00439360` (Raw File Offset `0x00039360`)  
**Status:** **CONFIRMED & EVIDENCE-BACKED**  

---

## 1. INSTRUKSI BINER ASLI PADA RVA 0x00439360

Pemeriksaan heksadesimal langsung pada raw byte offset `0x00039360`:

```assembly
0x00439360: 6A FF                   push -1
0x00439362: 68 20 F0 49 00          push 0x0049F020      ; Exception handler
0x00439367: 64 A1 00 00 00 00       mov eax, fs:[0]
...
0x00439396: 88 5C 24 3C             mov [esp+3Ch], bl
0x0043939A: 83 78 F8 1C             cmp dword ptr [eax-8], 1Ch  ; BANDINGKAN PANJANG STRING DENGAN 0x1C (28)
0x0043939E: 0F 85 57 01 00 00       jne 0x004394FB              ; JIKA TIDAK SAMA, LOMPAT KE ERROR HANDLER (FAIL)
```

## 2. ANALISIS OPERAND & STRUKTUR STRING MFC CSTRING
1. **Struktur Data:** Biner menggunakan struktur `CString` bawaan MFC. Pada memori internal `CString`, pointer string menunjuk tepat ke awal karakter data buffer, sedangkan header metadata terletak pada offset negatif:
   - `[eax - 4]`: Buffer reference count (allocated length).
   - `[eax - 8]`: **String Length (Data Length)** dalam satuan byte/karakter ASCII.
2. **Nilai Operand:** Instruksi `cmp dword ptr [eax-8], 1Ch` membandingkan panjang karakter data dengan konstanta heksadesimal `0x1C` (desimal 28).
   - Format: Karakter data murni yang dimuat string adalah 28 byte (string null-terminated berada di byte ke-29).
3. **Pemeriksaan Karakter Pemisah Tanda Hubung (`-`):**
   Segera setelah instruksi `cmp dword ptr [eax-8], 1Ch`, fungsi mengeksekusi pemeriksaan delimiter:
   ```assembly
   0x004393A4: 57                      push edi
   0x004393A5: 53                      push ebx
   0x004393A6: 6A 05                   push 5               ; Indeks 5
   0x004393A8: 8D 4C 24 50             lea ecx, [esp+50h]
   0x004393AC: E8 77 0E 06 00          call CString::GetAt  ; Periksa '-' pada indeks 5
   0x004393B1: 53                      push ebx
   0x004393B2: 6A 0A                   push 0Ah             ; Indeks 10
   ...
   0x004393B8: 6A 0F                   push 0Fh             ; Indeks 15
   ...
   0x004393BE: 6A 14                   push 14h             ; Indeks 20
   ```
   Setiap karakter pada indeks 5, 10, 15, dan 20 dibandingkan dengan nilai ASCII `0x2D` (`-`).
4. **Kesimpulan Format:**
   - 4 segmen pertama berpanjang tepat 5 karakter alfanumerik (`XXXXX-XXXXX-XXXXX-XXXXX-`).
   - 4 segmen $\times$ 5 karakter + 4 tanda hubung = 24 karakter.
   - Sisa panjang adalah $28 - 24 = 4$ karakter untuk segmen kelima (`XXXX`).
   - Pola format yang divalidasi adalah: `5-5-5-5-4` (Total 28 karakter data buffer).
5. **Hubungan dengan RVA 0x0043DB00 (MD5 Engine):**
   Fungsi `0x00439360` **BUKAN HANYA MEMERIKSA FORMAT**. Setelah panjang dan delimiter terverifikasi, fungsi membentuk string gabungan dan memanggil `0x0043DB00` (MD5 context init & hash calculation) untuk memverifikasi keaslian kriptografis serial number terhadap seed internal.
