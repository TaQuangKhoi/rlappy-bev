# GitHub Actions Release Workflow

Tài liệu này mô tả quy trình tự động hóa release cho dự án rlappy-bev.

## Cách sử dụng / How to Use

### Tạo Release tự động / Create Release Automatically

1. **Sử dụng GitHub Actions UI:**
   - Vào tab **Actions** trong GitHub repository
   - Chọn workflow **"Create Version Tag"**
   - Click **"Run workflow"**
   - Nhập version number (ví dụ: `v0.1.1`, `v1.0.0`)
   - Click **"Run workflow"**

2. **Hoặc sử dụng command line:**
   ```bash
   git tag -a v0.1.1 -m "Release v0.1.1"
   git push origin v0.1.1
   ```

### Quy trình tự động / Automatic Process

Khi tag được tạo, hệ thống sẽ tự động:
1. ✅ Tạo GitHub Release với tên version
2. ✅ Build game với cargo cho 3 nền tảng:
   - Windows (x86_64) → `rlappy-bev-windows-x86_64.exe`
   - Linux (x86_64) → `rlappy-bev-linux-x86_64`
   - macOS (x86_64) → `rlappy-bev-macos-x86_64`
3. ✅ Upload các file binary lên GitHub Release
4. ✅ Release sẵn sàng để download!

## Cấu trúc Workflow / Workflow Structure

### 1. Release Workflow (`.github/workflows/release.yml`)
- **Trigger:** Khi push tag có pattern `v*` (ví dụ: v0.1.0, v1.0.0)
- **Jobs:**
  - `create-release`: Tạo GitHub Release
  - `build`: Build binary cho Windows, Linux, và macOS
  
### 2. Create Tag Workflow (`.github/workflows/create-tag.yml`)
- **Trigger:** Manual (workflow_dispatch)
- **Input:** Version number (phải theo format `v0.0.0`)
- **Validation:** Kiểm tra format và tag đã tồn tại chưa

## Requirements

### Linux Build Requirements
Workflow sẽ tự động cài đặt:
- `pkg-config`
- `libx11-dev`
- `libasound2-dev`
- `libudev-dev`

### Rust Toolchain
- Sử dụng stable Rust toolchain
- Target platforms:
  - `x86_64-pc-windows-msvc`
  - `x86_64-unknown-linux-gnu`
  - `x86_64-apple-darwin`

## Versioning Guidelines

Dự án sử dụng Semantic Versioning:
- **v0.1.0**: Initial release
- **v0.1.x**: Bug fixes
- **v0.x.0**: Minor features
- **v1.0.0**: Major release

## Troubleshooting

### Workflow không chạy?
- Kiểm tra tag format phải bắt đầu với `v` (ví dụ: `v0.1.0`)
- Đảm bảo đã push tag lên GitHub: `git push origin <tag-name>`

### Build failed?
- Kiểm tra Cargo.toml dependencies
- Xem logs trong GitHub Actions để debug

### Upload failed?
- Đảm bảo GITHUB_TOKEN có đủ permissions
- Kiểm tra binary path trong workflow

## Future Improvements

Có thể cải tiến thêm:
- [ ] Build cho ARM64 (Apple Silicon)
- [ ] Build cho Linux ARM
- [ ] Tạo Windows installer (.msi)
- [ ] Upload lên crates.io
- [ ] Automatic changelog generation
- [ ] Discord/Slack notifications
