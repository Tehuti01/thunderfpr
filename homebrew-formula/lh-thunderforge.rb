class LhThunderforge < Formula
  desc "Professional Guitar Amp Simulator - VST3/CLAP Plugin for macOS"
  homepage "https://github.com/Tehuti01/thunderfpr"
  url "https://github.com/Tehuti01/thunderfpr/releases/download/v1.0.1/LH-Thunderforge-v1.0.0-macos.zip"
  sha256 "45ed749e39f0eba08c1ee82ea6ea6eb2c018df9b1cc65408508c2bc8558f8f96"
  version "1.0.0"

  head "https://github.com/Tehuti01/thunderfpr.git", branch: "main"

  depends_on "rust" => :build
  depends_on :macos => :high_sierra

  def install
    # Build from source
    system "cargo", "build", "--release"

    dylib = "target/release/libthunderforge_plugin.dylib"

    # VST3 bundle
    vst3_bundle = "#{lib}/VST3/LH Thunderforge.vst3/Contents"
    (Pathname.new("#{vst3_bundle}/MacOS")).mkpath
    (Pathname.new("#{vst3_bundle}/Resources")).mkpath

    cp dylib, "#{vst3_bundle}/MacOS/LH Thunderforge"
    chmod 0755, "#{vst3_bundle}/MacOS/LH Thunderforge"
    File.write("#{vst3_bundle}/PkgInfo", "BNDL????")

    (Pathname.new("#{vst3_bundle}/Info.plist")).write <<~EOS
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
          <key>CFBundleShortVersionString</key>
          <string>#{version}</string>
          <key>CFBundlePackageType</key>
          <string>BNDL</string>
          <key>CSResourcesFileMapped</key>
          <true/>
          <key>LSMinimumSystemVersion</key>
          <string>10.13</string>
          <key>NSHighResolutionCapable</key>
          <true/>
      </dict>
      </plist>
    EOS

    # CLAP bundle
    clap_bundle = "#{lib}/CLAP/LH Thunderforge.clap/Contents"
    (Pathname.new("#{clap_bundle}/MacOS")).mkpath

    cp dylib, "#{clap_bundle}/MacOS/LH Thunderforge"
    chmod 0755, "#{clap_bundle}/MacOS/LH Thunderforge"
    File.write("#{clap_bundle}/PkgInfo", "BNDL????")

    (Pathname.new("#{clap_bundle}/Info.plist")).write <<~EOS
      <?xml version="1.0" encoding="UTF-8"?>
      <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
      <plist version="1.0">
      <dict>
          <key>CFBundleExecutable</key>
          <string>LH Thunderforge</string>
          <key>CFBundleIdentifier</key>
          <string>capital.stradego.thunderforge.clap</string>
          <key>CFBundleName</key>
          <string>LH Thunderforge</string>
          <key>CFBundleVersion</key>
          <string>#{version}</string>
          <key>CFBundlePackageType</key>
          <string>BNDL</string>
      </dict>
      </plist>
    EOS

    # Standalone test app
    bin.install "target/release/thunderforge-standalone" => "lh-thunderforge"
  end

  def post_install
    # Symlink VST3 into user's plugin folder
    vst3_src = "#{lib}/VST3/LH Thunderforge.vst3"
    vst3_dst = File.expand_path("~/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3")
    clap_src  = "#{lib}/CLAP/LH Thunderforge.clap"
    clap_dst  = File.expand_path("~/Library/Audio/Plug-Ins/CLAP/LH Thunderforge.clap")

    FileUtils.mkdir_p(File.expand_path("~/Library/Audio/Plug-Ins/VST3"))
    FileUtils.mkdir_p(File.expand_path("~/Library/Audio/Plug-Ins/CLAP"))

    FileUtils.rm_rf(vst3_dst)
    FileUtils.ln_s(vst3_src, vst3_dst)

    FileUtils.rm_rf(clap_dst)
    FileUtils.ln_s(clap_src, clap_dst)
  end

  def caveats
    <<~EOS
      LH Thunderforge has been installed!

      VST3 plugin: ~/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3
      CLAP plugin: ~/Library/Audio/Plug-Ins/CLAP/LH Thunderforge.clap
      Standalone:  lh-thunderforge (run from terminal)

      To use in FL Studio:
        1. Open FL Studio
        2. Options → Manage plugins
        3. Scan for new plugins
        4. Find and load "LH Thunderforge" on a mixer track

      To use in Logic Pro / Reaper / Bitwig:
        Restart your DAW and scan for new plugins.

      To update:
        brew upgrade lh-thunderforge
    EOS
  end

  test do
    assert_predicate lib/"VST3/LH Thunderforge.vst3", :directory?
    assert_predicate lib/"CLAP/LH Thunderforge.clap", :directory?
  end
end
