# Task #6 최종 결과보고서

## HWP HostApp 첫 페이지 렌더링 누락 및 Quick Look 안정화

### 메타데이터

| 항목 | 값 |
|------|-----|
| 이슈 | `postmelee/rhwp#6` |
| 마일스톤 | `M001` — macOS Quick Look v0.1 안정화 |
| 작업 브랜치 | `local/task6` |
| 반영 대상 | 로컬 `macos/devel` |
| 작성일 | 2026-04-22 |

### 최종 상태

Issue #6은 현재까지 구현한 범위로 마무리한다. 원격 저장소 PR은 생성하지 않고, 로컬 `local/task6` 작업분을 로컬 `macos/devel`에 PR 방식의 `--no-ff` merge로 반영한다.

### 완료 범위

| 단계 | 상태 | 요약 |
|------|------|------|
| Stage 1 | 완료 | HostApp 첫 페이지 및 인접 페이지 선로드, 사이드바 제거 |
| Stage 2 | 완료 | Finder Thumbnail 줌 단계별 미표시 문제 수정 |
| Stage 3 | 완료 | Quick Look Preview 설치/등록 경로 복구 |
| Stage 4 | 완료 | 통합 검증 및 최종 보고 |

### 주요 변경

#### macOS HostApp

- 문서 로드 직후 첫 2페이지를 선로드한다.
- 현재 페이지 앞뒤 1페이지를 함께 로드한다.
- 첫 2페이지와 현재 페이지 주변 페이지는 즉시 unload하지 않도록 보호한다.
- HostApp 사이드바를 제거하고 문서 뷰어 단일 화면으로 정리했다.

#### Finder Thumbnail

- `QLThumbnailMinimumDimension`을 `64`에서 `16`으로 낮췄다.
- thumbnail drawing에서 clip bounds 안에 첫 페이지 이미지를 aspect-fit으로 배치한다.
- 흰 페이지 배경, 투명 외곽, 얇은 경계선, 고품질 interpolation을 적용했다.

#### Quick Look Preview

- 서명된 Debug 앱을 빌드해 `/Applications/HWP Quick Look.app`에 설치하는 스크립트를 추가했다.
- 설치 스크립트에서 signature 검증, LaunchServices 등록, Quick Look reset, `pluginkit` 등록 확인을 수행한다.
- Finder 스페이스바 Quick Look에서 `KTX.hwp` 첫 페이지가 표시됨을 확인했다.

#### iOS 설정 변경

- 사용자가 이번 브랜치에 포함하기로 한 iOS signing/bundle 설정 변경이 함께 포함되어 있다.
- `DEVELOPMENT_TEAM`은 `XH6JHKYXV8`로, bundle identifier는 `com.postmelee.alhangeul`로 변경되었다.
- iOS 앱 제품명은 `알한글.app`으로 정리되었다.

### 변경 파일

| 파일 | 내용 |
|------|------|
| `rhwp-macos/Sources/HostApp/Stores/DocumentViewerStore.swift` | 초기/인접 페이지 선로드 및 unload 보호 |
| `rhwp-macos/Sources/HostApp/Views/DocumentViewerView.swift` | page appear/task 시 인접 페이지 로드 |
| `rhwp-macos/Sources/HostApp/Views/ContentView.swift` | 사이드바 제거 |
| `rhwp-macos/Sources/HostApp/HostApp.swift` | 미사용 extension status 주입 제거 |
| `rhwp-macos/Sources/ThumbnailExtension/HwpThumbnailProvider.swift` | thumbnail aspect-fit drawing 보정 |
| `rhwp-macos/Sources/ThumbnailExtension/Info.plist` | `QLThumbnailMinimumDimension` 하향 |
| `rhwp-macos/scripts/install-debug-app.sh` | 서명 빌드/설치/등록 스크립트 |
| `rhwp-ios/AlHangeul.xcodeproj/project.pbxproj` | iOS signing/bundle 설정 변경 |
| `rhwp-ios/Sources/Info.plist` | plist 정렬 변경 |
| `mydocs/orders/20260422.md` | 오늘 할일 상태 갱신 |
| `mydocs/plans/task_m001_6.md` | 수행계획서 |
| `mydocs/plans/task_m001_6_impl.md` | 구현계획서 |
| `mydocs/working/task_m001_6_stage1.md` | Stage 1 완료보고서 |
| `mydocs/working/task_m001_6_stage2.md` | Stage 2 완료보고서 |
| `mydocs/working/task_m001_6_stage3.md` | Stage 3 완료보고서 |

### 검증 결과

| 검증 | 결과 |
|------|------|
| HostApp Debug 빌드 (`CODE_SIGNING_ALLOWED=NO`) | 통과 |
| 서명된 Debug 앱 빌드 및 `/Applications` 설치 | 통과 |
| `codesign --verify --deep --strict` | 통과 |
| `pluginkit` Preview extension 등록 | 통과 (`com.postmelee.rhwpmac.QLExtension`) |
| `pluginkit` Thumbnail extension 등록 | 통과 (`com.postmelee.rhwpmac.ThumbnailExtension`) |
| `KTX.hwp` 첫 페이지 PNG 렌더 체크 | 통과 (`nonWhitePixels=450455`) |
| QuickLookThumbnailing API 썸네일 생성 | 통과 (`0.047s`, `512x363`) |
| Finder 아이콘 보기 16/32/64 단계 | 통과 |
| Finder 스페이스바 Quick Look | 통과 |
| `install-debug-app.sh` 문법 검사 | 통과 |

### 실행 명령

```bash
xcodebuild -project rhwp-macos/RhwpMacOS.xcodeproj \
  -scheme HostApp \
  -configuration Debug \
  -derivedDataPath rhwp-macos/build/DerivedData \
  CONFIGURATION_BUILD_DIR=/Users/melee/Documents/projects/rhwp-macos/rhwp-macos/build/debug \
  CODE_SIGNING_ALLOWED=NO \
  build

rhwp-macos/scripts/install-debug-app.sh

/tmp/stage5_thumbnail_check \
  /Users/melee/Documents/projects/rhwp-macos/samples/basic/KTX.hwp \
  /tmp/KTX-stage6-thumbnail.png

rhwp-macos/scripts/validate-stage3-render.sh \
  /tmp/stage6-render-check \
  /Users/melee/Documents/projects/rhwp-macos/samples/basic/KTX.hwp

bash -n rhwp-macos/scripts/install-debug-app.sh
```

### 남은 이슈

- `qlmanage -t` CLI는 macOS 26.4.1 환경에서 기존 Stage 5와 동일하게 종료되지 않는 현상이 있다. 사용자-facing 검증은 Finder UI와 QuickLookThumbnailing API 기준으로 통과했다.
- Stage 7 배포 패키징과 Homebrew cask 준비는 Issue #5로 보류한다.
- Issue #6 GitHub issue close는 작업지시자 승인 전에는 수행하지 않는다.
