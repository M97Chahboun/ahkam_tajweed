# TAJWEED PROCESSOR - QUICK REFERENCE GUIDE

## Rules Implemented by Category

### ✅ FULLY OPERATIONAL (25 Rules)

#### Noon Sakinah & Tanwin (8)
- Izhar Halqi (throat letters)
- Izhar Mutlaq (ya/wa exception)
- Idgham Bi-Ghunnah (ي ن م و)
- Idgham Bila-Ghunnah (ل ر)
- Iqlab (before ب)
- Ikhfaa Haqiqi (15 letters)
- Idgham Naqis (Warsh)
- Idgham Kamil

#### Mim Sakinah (3)
- Ikhfaa Shafawi (before ب)
- Idgham Shafawi (before م)
- Izhar Shafawi (other letters)

#### Lam Al-Taref (2)
- Izhar Qamari (14 lunar)
- Idgham Shamsi (14 solar)

#### Madd Rules (7)
- Madd Tabeei (natural)
- Madd Muttasil (same word hamza)
- Madd Munfasil (next word hamza)
- Madd Lazim (shadda)
- Madd Badal (hamza→alif)
- Madd Arid (word end)
- Madd Lin (soft)

#### Qalqalah (2)
- Qalqalah Kubra (word end)
- Qalqalah Sughra (middle)

### ⏳ DEFINED BUT NOT YET DETECTED (3 Rules)
- Tarqeeq Ra (thinnification)
- Tafkhim Ra (emphasis)
- Tafkhim Lafz Al-Jalalah

---

## Test Commands

### Single Verse
```bash
echo "الحَمْدُ" | ./target/debug/tajweed_warsh_rules
```

### Multiple Verses
```bash
echo -e "آمَنَ\nقَالَ\nبِدْ" | ./target/debug/tajweed_warsh_rules
```

### Interactive Mode
```bash
./target/debug/tajweed_warsh_rules
:style warsh
قُرْآنٌ
:style both
آمَنُوا
q
```

---

## Key Detection Rules

### Qalqalah Requirements
- ✅ Must have SUKUN (U+0652)
- Letters: ق ط ب ج د
- Kubra: At word/verse end
- Sughra: In word middle

### Madd Requirements
- ✅ Must have correct vowel before:
  - Alif: Fatha (U+064E)
  - Waaw: Damma (U+064C)
  - Ya: Kasra (U+0650)

### Noun/Mim Sakinah Requirements
- ✅ Must have SUKUN (U+0652) OR TANWIN (U+064B/C/D)

---

## Examples

| Text   | Rules Detected              |
| ------ | --------------------------- |
| الحَمْدُ  | Izhar Qamari, Izhar Shafawi |
| آمَنَ    | Madd Badal                  |
| قَالَ    | Madd Tabeei                 |
| بِدْ     | Qalqalah Kubra              |
| الصِّرَاطَ | Idgham Shamsi               |
| أَنْعَمْتَ  | Izhar Halqi                 |

