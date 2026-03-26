# 🍺 Homebrew Tap for LH Thunderforge

## Installation

```bash
# Add tap
brew tap Tehuti01/threwfpr

# Install
brew install lh-thunderforge

# Update
brew upgrade lh-thunderforge

# Uninstall
brew uninstall lh-thunderforge
```

## Formula Location

The formula should be placed at:
```
https://github.com/Tehuti01/homebrew-threwfpr/blob/main/Formula/lh-thunderforge.rb
```

## Creating Homebrew Tap Repository

1. Create new repo: `homebrew-threwfpr`
2. Add formula to `Formula/lh-thunderforge.rb`
3. Users can then: `brew tap Tehuti01/threwfpr`

## Formula

```ruby
class LhThunderforge < Formula
  desc "Professional Guitar Amp Simulator VST3"
  homepage "https://github.com/Tehuti01/thunderfpr"
  url "https://github.com/Tehuti01/thunderfpr/releases/download/v1.0.0/LH-Thunderforge-macos.zip"
  sha256 :no_check
  version "1.0.0"

  def install
    mkdir_p "#{lib}/VST3/LH Thunderforge.vst3/Contents/MacOS"
    system "unzip", "LH-Thunderforge-macos.zip"
    mv "LH Thunderforge.vst3/Contents/MacOS/LH Thunderforge", "#{lib}/VST3/LH Thunderforge.vst3/Contents/MacOS/"
  end
end
```
