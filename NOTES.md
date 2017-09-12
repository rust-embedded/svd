
End to end tests require some normalisation of files to correct for varying formatting by vendors (because the output format is static).

- Normalise hexadecimal case: `gsed -i="" 's|\(0x[0-9a-fA-F]*\)|\L\1|g' FILENAME`
- Normalise hexadecimal lengths: `gsed -i"" 's|>0x[0]\{1,2\}\([0-9a-f]\{1,2\}\)<|>0x\1<|g' FILENAME`

