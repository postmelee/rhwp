# Task #6 — Stage 1 완료보고서

## HostApp 초기 페이지 렌더링 안정화 및 사이드바 제거

### 메타데이터

| 항목 | 값 |
|------|-----|
| 이슈 | `postmelee/rhwp#6` |
| 마일스톤 | `M001` — macOS Quick Look v0.1 안정화 |
| 작업 브랜치 | `local/task6` |
| 기준 브랜치 | `macos/devel` |
| 단계 | Stage 1 |
| 작성일 | 2026-04-22 |

### 작업 배경

`HWP Quick Look.app`으로 `.hwp` 파일을 열었을 때 첫 페이지가 흰 화면 또는 로딩 상태로 남는 문제가 확인되었다. `KTX.hwp` 실제 실행 화면에서 문서명과 페이지 수는 표시되지만 페이지 내용이 보이지 않는 상황이 재현되었다.

추가 확인 결과, 첫 페이지 PNG 렌더링 경로 자체는 정상이고 HostApp Viewer의 SwiftUI lazy loading 및 페이지 캐시 정책이 초기 화면 표시를 지연시키는 것으로 판단했다.

### 수행 내용

1. `DocumentViewerStore`에 `initialPreloadPageCount = 2`를 추가했다.
2. 문서 로드 직후 첫 2페이지를 선로드하도록 `preloadInitialPages()`를 추가했다.
3. `loadPage(_:)`에 페이지 범위 검증을 추가했다.
4. `unloadPage(_:)`에서 첫 2페이지와 현재 페이지 주변 페이지를 즉시 언로드하지 않도록 보호했다.
5. 현재 페이지 앞뒤 1페이지를 함께 로드하는 `loadPages(around:)`를 추가했다.
6. `DocumentPageContainer`에서 `onAppear`와 `.task(id:)` 양쪽으로 인접 페이지 로드를 호출하도록 보강했다.
7. HostApp 좌측 사이드바를 제거하고 문서 뷰어를 단일 루트 콘텐츠로 배치했다.
8. `HostApp.swift`에서 더 이상 쓰지 않는 `ExtensionStatusModel` 주입/refresh를 제거했다.

### 변경 파일

| 파일 | 변경 |
|------|------|
| `rhwp-macos/Sources/HostApp/Stores/DocumentViewerStore.swift` | 초기/인접 페이지 선로드 및 언로드 보호 |
| `rhwp-macos/Sources/HostApp/Views/DocumentViewerView.swift` | 페이지 appear/task 시 인접 페이지 로드 |
| `rhwp-macos/Sources/HostApp/Views/ContentView.swift` | 사이드바 제거, 문서 뷰 단일 루트화 |
| `rhwp-macos/Sources/HostApp/HostApp.swift` | 미사용 extension status 상태 제거 |

### 검증 결과

| 항목 | 결과 |
|------|------|
| HostApp Debug 빌드 | 통과 |
| `samples/basic/KTX.hwp` 첫 페이지 PNG 렌더 체크 | 통과 (`nonWhitePixels=450455`) |
| 번들 `sample.hwpx` 첫 페이지 PNG 렌더 체크 | 통과 (`nonWhitePixels=35387`) |
| 수정 빌드 재시작 후 `KTX.hwp` 실제 앱 표시 | 통과 |
| 사이드바 제거 후 `KTX.hwp` 실제 앱 표시 | 통과 |

### 실행 명령

```bash
xcodebuild -project rhwp-macos/RhwpMacOS.xcodeproj \
  -scheme HostApp \
  -configuration Debug \
  -derivedDataPath rhwp-macos/build/DerivedData \
  CONFIGURATION_BUILD_DIR=/Users/melee/Documents/projects/rhwp-macos/rhwp-macos/build/debug \
  CODE_SIGNING_ALLOWED=NO \
  build

rhwp-macos/scripts/validate-stage3-render.sh \
  /Users/melee/Documents/projects/rhwp-macos/output/task6-render \
  /Users/melee/Documents/projects/rhwp-macos/samples/basic/KTX.hwp \
  /Users/melee/Documents/projects/rhwp-macos/rhwp-macos/Sources/HostApp/Resources/sample.hwpx

open -a /Users/melee/Documents/projects/rhwp-macos/rhwp-macos/build/debug/HWP\ Quick\ Look.app \
  /Users/melee/Documents/projects/rhwp-macos/samples/basic/KTX.hwp
```

### 특이사항

- `xcodebuild` 중 CoreSimulator 관련 경고가 출력되었지만 macOS HostApp 빌드는 성공했다.
- 현재 빌드 산출물은 `CODE_SIGNING_ALLOWED=NO`로 생성한 검증용 Debug 빌드다.
- Finder Thumbnail과 Quick Look Preview 미동작 문제는 Stage 2/3에서 별도 승인 후 원인을 분리한다.
- 이 보고서는 이미 수행한 Stage 1 변경을 AGENTS.md 절차에 맞춰 소급 정리한 canonical 단계 보고서다.
