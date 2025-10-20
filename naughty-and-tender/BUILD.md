# Naughty and Tender - Build and Testing Guide

## Building the Plugin

### Prerequisites

1. **Rust toolchain** - Install from https://rustup.rs/
2. **Git** - For cloning nih-plug dependencies
3. **DAW** - Reaper, Bitwig, or any VST3/CLAP-compatible DAW

### Build Commands

#### Development Build
```bash
# From the root of the audio-experiments repository
cd xtask
cargo run --release -- bundle naughty-and-tender
```

#### Release Build
```bash
cd xtask
cargo run --release -- bundle naughty-and-tender --release
```

#### Check Only (No Bundle)
```bash
# From the root directory
cargo check --package naughty-and-tender
```

### Build Outputs

Successful builds create:
- **VST3 Bundle**: `target/bundled/naughty-and-tender.vst3`
- **CLAP Bundle**: `target/bundled/naughty-and-tender.clap`

## Installing in Reaper

### Windows Installation

1. **Locate your Reaper VST3 folder**:
   - Default: `C:\Program Files\Common Files\VST3\`
   - Or check Reaper → Preferences → Plug-ins → VST → VST3 path

2. **Copy the plugin**:
   ```bash
   # From the project root
   copy target\bundled\naughty-and-tender.vst3 "C:\Program Files\Common Files\VST3\"
   ```

3. **Rescan plugins in Reaper**:
   - Options → Preferences → Plug-ins → VST
   - Click "Re-scan"
   - Or: "Clear cache/re-scan all"

### Linux/Mac Installation

**Linux**:
```bash
cp -r target/bundled/naughty-and-tender.vst3 ~/.vst3/
```

**Mac**:
```bash
cp -r target/bundled/naughty-and-tender.vst3 ~/Library/Audio/Plug-Ins/VST3/
```

## Testing in Reaper

### Basic Load Test

1. **Create a new MIDI track**
2. **Add the plugin**:
   - Track → Insert virtual instrument on new track
   - Search for "Naughty and Tender"
   - Add it

3. **Verify it loads**:
   - GUI should appear with status information
   - No errors should appear in Reaper's console

### MIDI Input Test

1. **Enable MIDI recording** on the track
2. **Play some MIDI notes** (via MIDI keyboard or MIDI editor)
3. **Check the logs**:
   - On Windows: Run `DebugView` from Sysinternals
   - Or check Reaper's console (View → Monitoring FX/Track output)
   - You should see "Note On" and "Note Off" messages

Expected log output:
```
Naughty and Tender initialized
Sample rate: 48000
Max buffer size: 512
Note On: 60 velocity: 0.8
Note Off: 60
```

### Parameter Test

1. **Open the plugin GUI**
2. **Move the Gain slider**
3. **Verify**:
   - Slider responds smoothly
   - Parameter is automatable (Param → Show track envelope → Gain)

### Audio Callback Test

**Current Behavior** (Phase 1):
- Plugin outputs **silence** (this is expected!)
- No crashes or glitches
- Audio callback is being called correctly

**To verify audio callback**:
1. Add plugin to a MIDI track
2. Play a MIDI note
3. Check CPU meter - should show minimal usage
4. No audio artifacts or pops/clicks

## Troubleshooting

### Plugin doesn't appear in Reaper

**Possible causes**:
1. Plugin not copied to VST3 folder
2. Reaper didn't rescan plugins
3. Plugin crashed on load

**Solutions**:
- Verify file exists in VST3 folder
- Clear plugin cache and re-scan
- Check Reaper error log: View → Monitoring FX/Track output

### Build errors

**nih-plug git dependencies fail**:
```bash
# Clear cargo cache and retry
cargo clean
rm -rf ~/.cargo/git/checkouts/
cargo check --package naughty-and-tender
```

**Compilation errors**:
- Ensure Rust is up to date: `rustup update`
- Check for missing system dependencies (Linux: `build-essential`, `libasound2-dev`)

### GUI doesn't appear

**Check**:
- GUI state parameter is persisting correctly
- egui-baseview compiled successfully
- No errors in build output

## Development Workflow

### Recommended Workflow

1. **Make code changes** in your editor
2. **Build plugin**:
   ```bash
   cd xtask && cargo run --release -- bundle naughty-and-tender
   ```
3. **Close plugin in Reaper** (important!)
4. **Copy new bundle** to VST3 folder:
   ```bash
   copy target\bundled\naughty-and-tender.vst3 "C:\Program Files\Common Files\VST3\" /Y
   ```
5. **Reload plugin in Reaper**
6. **Test changes**

### Hot Reloading Tips

- **Always close the plugin** before copying new version
- Reaper can lock the DLL, preventing overwrite
- Consider using a batch script for build→copy workflow

### Debugging

**View debug logs**:
- Windows: Use [DebugView](https://learn.microsoft.com/en-us/sysinternals/downloads/debugview)
- Linux: `journalctl --user -f | grep naughty`
- Use `nih_log!()` macro in code

**Attach debugger**:
- Rust-specific debugging with `rust-lldb` or `rust-gdb`
- Set breakpoints in your IDE (VS Code with rust-analyzer)

## CI/CD Recommendations

### GitHub Actions Example

```yaml
name: Build Plugin

on: [push, pull_request]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Build plugin
      run: |
        cd xtask
        cargo run --release -- bundle naughty-and-tender --release

    - name: Upload artifact
      uses: actions/upload-artifact@v3
      with:
        name: naughty-and-tender-${{ matrix.os }}
        path: target/bundled/
```

## Performance Testing

### CPU Usage Target

**Phase 1 (Current)**:
- Target: <1% CPU with 8 voices
- Current: Minimal (just silence generation)

**Future Phases**:
- Phase 2 (Oscillators): <5% CPU with 8 voices
- Phase 3 (Filters + Modulation): <10% CPU with 8 voices

### Profiling

```bash
# Build with profiling symbols
cargo xtask bundle naughty-and-tender --profile profiling

# Use flamegraph (if installed)
cargo flamegraph --package naughty-and-tender
```

## Validation

### pluginval

[pluginval](https://github.com/Tracktion/pluginval) is the standard VST3 validation tool.

**Windows**:
```bash
pluginval --strictness-level 10 --validate "target\bundled\naughty-and-tender.vst3"
```

**Expected Results** (Phase 1):
- ✅ Plugin loads
- ✅ Parameters validated
- ✅ MIDI input functional
- ✅ Audio processing (silence)
- ⚠️ No actual sound output (expected in Phase 1)

## Next Steps

After successful Phase 1 testing:

1. **Phase 2**: Add oscillators and basic synthesis
2. **Phase 3**: Add envelope and modulation
3. **Phase 4**: Add filters and effects
4. **Phase 5**: Polish and optimize

---

**Current Status**: Phase 1 Complete ✅
- Plugin loads in DAW
- MIDI events processed
- Parameters functional
- Audio callback active
- Ready for Phase 2 (Oscillator implementation)
