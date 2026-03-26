# GitHub Setup Guide - LH Thunderforge

## 🚀 Creating Your GitHub Repository

### Step 1: Create Repository on GitHub

1. Go to **https://github.com/new**
2. Repository name: `thunderforge`
3. Owner: `Tehuti01`
4. Description: "Professional Guitar Amp Simulator & Neural Modeler - VST3/CLAP Plugin for macOS"
5. **Public** repository (so friends can download)
6. **DO NOT** initialize with README (we already have one)
7. Click **"Create repository"**

### Step 2: Push Code to GitHub

After creating the repository, run these commands:

```bash
cd /Users/tehuti01/Desktop/Tehuti-vst-rust

# Add GitHub as remote
git remote add origin https://github.com/Tehuti01/thunderforge.git

# Push to GitHub
git branch -M main
git push -u origin main
```

### Step 3: Create First Release

#### Option A: Using GitHub CLI (Recommended)

```bash
# Install GitHub CLI if not installed
brew install gh

# Authenticate
gh auth login

# Create release
gh release create v1.0.0 \
  build/LH-Thunderforge-macos.zip \
  build/LH-Thunderforge-CLAP-macos.zip \
  --title "LH Thunderforge v1.0.0" \
  --notes "Initial release - See CHANGELOG.md for details" \
  --draft  # Remove --draft when ready to publish
```

#### Option B: Using GitHub Website

1. Go to your repository: **https://github.com/Tehuti01/thunderforge**
2. Click **"Releases"** on the right sidebar
3. Click **"Create a new release"**
4. Tag version: `v1.0.0`
5. Release title: `LH Thunderforge v1.0.0`
6. Description: Copy from CHANGELOG.md
7. Upload files:
   - `build/LH-Thunderforge-macos.zip`
   - `build/LH-Thunderforge-CLAP-macos.zip`
8. Check **"Set as latest release"**
9. Click **"Publish release"**

### Step 4: Share with Friends

Share these links with your friends:

**Main Repository:**
```
https://github.com/Tehuti01/thunderforge
```

**Direct Download (Latest Release):**
```
https://github.com/Tehuti01/thunderforge/releases/latest
```

**Auto-Install Command:**
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/install.sh | bash
```

**Auto-Update Command:**
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/update.sh | bash
```

---

## 📬 Notifying Friends About Updates

### Option 1: GitHub Notifications

Friends can **Watch** the repository to get notified:
1. Go to repository
2. Click **"Watch"** button (bell icon)
3. Select **"Releases"** to only get release notifications

### Option 2: Discord/Slack Integration

Set up GitHub integration in your Discord server:
1. Server Settings → Integrations → GitHub
2. Connect repository
3. Select channel for release notifications

### Option 3: Email List

Create a simple mailing list:
- Use GitHub Discussions for announcements
- Or use a service like Mailchimp

---

## 🔄 Update Workflow

### When You Release a New Version:

1. **Update CHANGELOG.md** with new version
2. **Update version** in README.md
3. **Build release:**
   ```bash
   ./scripts/release.sh v1.1.0
   ```
4. **Commit changes:**
   ```bash
   git add -A
   git commit -m "Release v1.1.0 - [brief description]"
   git push
   ```
5. **Create GitHub release** (see Step 3 above)

Friends can then update with:
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/update.sh | bash
```

---

## 📊 Repository Stats

After pushing, your repository will show:
- **Stars** - Friends can star to support
- **Forks** - Others can fork to contribute
- **Watchers** - People following updates
- **Releases** - Downloadable versions
- **Issues** - Bug reports and feature requests
- **Discussions** - Community conversations

---

## 🔗 Quick Links

Once set up, share these:

| Link | Purpose |
|------|---------|
| `https://github.com/Tehuti01/thunderforge` | Main repository |
| `https://github.com/Tehuti01/thunderforge/releases` | All releases |
| `https://github.com/Tehuti01/thunderforge/issues` | Bug reports |
| `https://github.com/Tehuti01/thunderforge/discussions` | Community |

---

## ✅ Checklist

- [ ] Create repository on GitHub
- [ ] Push code with `git push`
- [ ] Create first release (v1.0.0)
- [ ] Upload release ZIP files
- [ ] Test install script
- [ ] Share with friends
- [ ] Set up notifications

---

**Your plugin will be live and ready for downloads! 🎸**
