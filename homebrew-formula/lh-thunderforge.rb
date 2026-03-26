class LhThunderforge < Formula
  desc "Professional Guitar Amp Simulator - VST3 Plugin for macOS"
  homepage "https://github.com/Tehuti01/thunderfpr"
  url "https://github.com/Tehuti01/thunderfpr/releases/download/v1.0.0/LH-Thunderforge-macos.zip"
  sha256 :no_check
  version "1.0.0"

  depends_on "rust" => :build

  def install
    # Build from source
    system "cargo", "build", "--release"
    
    # Create VST3 bundle
    vst3_dir = "#{lib}/VST3/LH Thunderforge.vst3/Contents"
    mkdir_p "#{vst3_dir}/MacOS"
    mkdir_p "#{vst3_dir}/Resources"
    
    # Copy binary
    cp "target/release/libthunderforge_plugin.dylib", "#{vst3_dir}/MacOS/LH Thunderforge"
    
    # Create Info.plist
    (vst3_dir/"Info.plist").write <<-EOS
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>LH Thunderforge</string>
    <key>CFBundleIdentifier</key>
    <string>capital.stradego.thunderforge</string>
    <key>CFBundleName</key>
    <string>LH Thunderforge</string>
    <key>CFBundleVersion</key>
    <string>#{version}</string>
</dict>
</plist>
EOS
  end

  def caveats
    <<~EOS
      LH Thunderforge VST3 plugin has been installed to:
        #{HOMEBREW_PREFIX}/lib/VST3/
      
      To use in FL Studio:
      1. Open FL Studio
      2. Options → Manage plugins
      3. Add folder: #{HOMEBREW_PREFIX}/lib/VST3/
      4. Scan for plugins
      5. Load "LH Thunderforge" on a mixer track
    EOS
  end

  test do
    assert_predicate lib/"VST3/LH Thunderforge.vst3", :directory?
  end
end
