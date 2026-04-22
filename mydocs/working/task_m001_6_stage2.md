# Task #6 — Stage 2 완료보고서

## Finder Thumbnail 줌 단계별 미표시 문제 정의 및 수정

### 메타데이터

| 항목 | 값 |
|------|-----|
| 이슈 | `postmelee/rhwp#6` |
| 마일스톤 | `M001` — macOS Quick Look v0.1 안정화 |
| 작업 브랜치 | `local/task6` |
| 기준 브랜치 | `macos/devel` |
| 단계 | Stage 2 |
| 작성일 | 2026-04-22 |

### 문제 정의

Finder 아이콘 보기에서 특정 줌 단계의 `.hwp` 썸네일이 보이지 않았다. 너무 작거나 너무 커서 보이지 않는 문제가 아니라, 줌을 조정하면 표시/미표시가 바뀌는 형태였다.

원인은 두 가지로 판단했다.

1. Thumbnail extension의 `QLThumbnailMinimumDimension`이 `64`로 설정되어 있어 Finder가 작은 아이콘 단계에서 provider 호출을 건너뛸 수 있었다.
2. drawing block이 Quick Look이 제공한 clip bounds를 그대로 사용하면서 page image의 aspect-fit 배치와 배경/경계 처리가 부족해 특정 context 크기에서 결과가 불안정했다.

### 수행 내용

1. `QLThumbnailMinimumDimension`을 `64`에서 `16`으로 낮춰 Finder 아이콘 보기의 작은 줌 단계도 thumbnail provider 대상에 포함했다.
2. `HwpThumbnailProvider.drawPageImage`에서 실제 clip bounds 안에 page image를 aspect-fit으로 중앙 배치하도록 보정했다.
3. 투명 배경, 흰 페이지 배경, 고품질 interpolation, 얇은 페이지 경계선을 추가해 작은 아이콘에서도 빈 썸네일처럼 보이지 않도록 했다.

### 변경 파일

| 파일 | 변경 |
|------|------|
| `rhwp-macos/Sources/ThumbnailExtension/Info.plist` | `QLThumbnailMinimumDimension` 16으로 하향 |
| `rhwp-macos/Sources/ThumbnailExtension/HwpThumbnailProvider.swift` | clip bounds 기반 aspect-fit drawing 및 페이지 경계 보정 |

### 검증 결과

| 항목 | 결과 |
|------|------|
| HostApp Debug 빌드 | 통과 |
| QuickLookThumbnailing API 썸네일 생성 | 통과 (`0.047s`, `512x363`, `/tmp/KTX-stage6-thumbnail.png`) |
| Finder 아이콘 보기 16 단계 | 통과 (`KTX.hwp` 썸네일 표시) |
| Finder 아이콘 보기 32 단계 | 통과 (`KTX.hwp` 썸네일 표시) |
| Finder 아이콘 보기 64 단계 | 통과 (`KTX.hwp` 썸네일 표시) |
| 기존 첫 페이지 PNG 렌더 체크 | 통과 (`nonWhitePixels=450455`) |

### 실행 명령

```bash
xcodebuild -project rhwp-macos/RhwpMacOS.xcodeproj \
  -scheme HostApp \
  -configuration Debug \
  -derivedDataPath rhwp-macos/build/DerivedData \
  CONFIGURATION_BUILD_DIR=/Users/melee/Documents/projects/rhwp-macos/rhwp-macos/build/debug \
  CODE_SIGNING_ALLOWED=NO \
  build

/tmp/stage5_thumbnail_check \
  /Users/melee/Documents/projects/rhwp-macos/samples/basic/KTX.hwp \
  /tmp/KTX-stage6-thumbnail.png

rhwp-macos/scripts/validate-stage3-render.sh \
  /tmp/stage6-render-check \
  /Users/melee/Documents/projects/rhwp-macos/samples/basic/KTX.hwp
```

### 특이사항

- `qlmanage -t -s 128 -o /tmp ...`는 기존 Stage 5와 동일하게 CLI 프로세스가 종료되지 않아 중단했다. 실제 Finder 아이콘 보기와 `QLThumbnailGenerator` API 경로는 정상이다.
- Finder 캐시 영향이 있을 수 있어, 검증 전 서명된 앱 설치와 `qlmanage -r` 등록 갱신을 수행했다.
