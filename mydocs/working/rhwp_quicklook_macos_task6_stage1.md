# rhwp-macos Quick Look — Task #6 Stage 1 완료 보고서

> 절차 보강 후 canonical 단계 보고서는 `mydocs/working/task_m001_6_stage1.md`이다. 이 문서는 작업 중 작성한 초기 보고서로 보존한다.

## 개요

- 작성 시각: 2026-04-22 KST
- 대상 이슈: `postmelee/rhwp#6`
- 작업 브랜치: `local/task6`
- 단계: HWP HostApp 첫 페이지 렌더링 누락 수정

## 문제

`HWP Quick Look.app`으로 `.hwp` 파일을 열었을 때 문서와 페이지 수는 인식되지만, 첫 페이지가 흰 페이지 또는 로딩 표시 상태로 남는 문제가 있었다. 일부 다중 페이지 문서에서는 두 번째 페이지도 늦게 보이거나 빈 상태로 남을 수 있었다.

## 원인 판단

HostApp Viewer는 `LazyVStack`에서 페이지가 나타난 뒤 `onAppear`로 렌더 트리를 생성했다. 앱 실행 직후 첫 페이지 렌더 트리가 준비되기 전에는 `ProgressView`가 먼저 표시되고, SwiftUI lazy view 수명주기와 스크롤 상태에 따라 초기 페이지나 인접 페이지 로딩이 늦어질 수 있었다.

Finder Quick Look Preview/Thumbnail 경로는 `HwpPageImageRenderer.renderFirstPage`가 첫 페이지를 동기 PNG로 생성하므로, 이번 증상은 HostApp Viewer의 페이지 캐시/선로드 정책 문제로 분리했다.

## 수정 내용

1. `DocumentViewerStore`가 문서 로드 직후 첫 2페이지 렌더 트리를 선로드하도록 변경했다.
2. 첫 2페이지는 `onDisappear`에서 언로드하지 않아 앱 시작 직후 다시 빈 상태로 돌아가지 않게 했다.
3. 화면에 나타난 페이지의 앞뒤 1페이지를 함께 로드하는 `loadPages(around:)`를 추가했다.
4. `DocumentPageContainer`에서 `onAppear`와 `.task(id:)` 양쪽으로 인접 페이지 선로드를 호출하도록 보강했다.
5. HostApp의 좌측 사이드바를 제거하고 문서 뷰어를 단일 루트 콘텐츠로 배치했다. 문서명, 현재 페이지, 확대율은 하단 상태바와 툴바에서 계속 확인할 수 있다.

## 검증 결과

| 항목 | 결과 |
|------|------|
| HostApp Debug 빌드 | 통과 |
| `samples/basic/KTX.hwp` 첫 페이지 PNG 렌더 체크 | 통과 (`nonWhitePixels=450455`) |
| 번들 `sample.hwpx` 첫 페이지 PNG 렌더 체크 | 통과 (`nonWhitePixels=35387`) |
| 수정 빌드 재시작 후 `KTX.hwp` 실제 앱 표시 | 통과 |
| 사이드바 제거 후 `KTX.hwp` 실제 앱 표시 | 통과 |

## 실행 명령

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

## 특이사항

- `xcodebuild` 중 CoreSimulator 관련 경고가 출력되었지만 macOS HostApp 빌드는 성공했다.
- 현재 빌드 산출물은 `CODE_SIGNING_ALLOWED=NO`로 생성한 검증용 Debug 빌드다. Finder Quick Look 확장 등록 상태 검증은 서명/설치 흐름에서 별도 확인이 필요하다.
