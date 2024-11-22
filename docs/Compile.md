## Compile Steps
### Windows build:
CLI:
```powershell
cargo build --release --target=x86_64-pc-windows-gnu
```

InnoSetup script:
```
iscc .\scripts\ci\installer.iss
```
