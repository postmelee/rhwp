# Task #6 — 구현계획서

## macOS Quick Look: 첫 페이지 렌더링 누락 및 Finder 통합 안정화

### 설계 개요

Task #6은 HostApp Viewer 경로와 Finder Extension 경로를 분리해서 검증한다.

```
HostApp Viewer
  DocumentViewerStore
  DocumentViewerView
  DocumentPageView
  CGTreeRenderer

Finder Quick Look Preview
  QLPreviewProvider
  HwpPageImageRenderer
  CGTreeRenderer

Finder Thumbnail
  QLThumbnailProvider
  HwpPageImageRenderer
  QLThumbnailReply drawing block
```

HostApp은 SwiftUI view life-cycle과 페이지 캐시 정책이 핵심이고, Finder 경로는 extension 등록/서명/UTI/provider 응답이 핵심이다. 두 경로를 한 번에 추정하지 않고 단계별로 원인을 분리한다.

### 구현 단계

---

#### Stage 1 — HostApp 초기 페이지 렌더링 안정화

상태: 완료

작업:
1. 문서 로드 직후 첫 2페이지 렌더 트리 선로드
2. 첫 2페이지는 `onDisappear`에서 즉시 언로드하지 않도록 보호
3. 현재 페이지 주변 앞뒤 1페이지를 함께 로드하는 `loadPages(around:)` 추가
4. `DocumentPageContainer`에서 `onAppear`와 `.task(id:)` 양쪽으로 인접 페이지 로드 호출
5. 사이드바 제거 및 문서 뷰어 단일 루트화

변경 파일:
- `rhwp-macos/Sources/HostApp/Stores/DocumentViewerStore.swift`
- `rhwp-macos/Sources/HostApp/Views/DocumentViewerView.swift`
- `rhwp-macos/Sources/HostApp/Views/ContentView.swift`
- `rhwp-macos/Sources/HostApp/HostApp.swift`

검증:
- HostApp Debug 빌드
- `validate-stage3-render.sh` 첫 페이지 PNG 렌더 체크
- 실제 앱 실행 후 `KTX.hwp` 첫 페이지 표시 확인
- 사이드바 제거 후 단일 문서 뷰 표시 확인

---

#### Stage 2 — Finder Thumbnail 줌 단계별 미표시 문제 정의 및 수정

상태: 완료

작업:
1. Finder 아이콘 뷰 줌 단계별 재현 조건 기록
2. `QLThumbnailReply(contextSize:)`와 drawing block의 실제 clipping bounds 비교
3. `request.maximumSize`, `request.minimumSize`, `request.scale` 처리 확인
4. Thumbnail drawing rect가 특정 줌에서 0 또는 비정상 rect가 되는지 검증
5. 필요 시 thumbnail context size 산정과 draw rect 산정을 보정

검증:
- Finder 아이콘 뷰 16/32/64 단계에서 HWP 썸네일 표시 확인
- QuickLookThumbnailing API 기반 회귀 체크 통과
- 기존 PNG 렌더 체크 회귀 없음

승인 필요:
- 2026-04-22 작업지시자의 "문제를 정의하고 해결" 요청을 Stage 2/3 진행 승인으로 간주

---

#### Stage 3 — Quick Look Preview 미동작 원인 분리 및 복구

상태: 완료

작업:
1. `pluginkit` 등록 상태와 QL extension discovery 상태 확인
2. HostApp 설치/실행 후 Preview extension 등록 경로 확인
3. `Info.plist`의 `NSExtension`, `QLSupportedContentTypes`, UTI 선언 확인
4. `HwpPreviewProvider.providePreview` 호출 여부 로그/수동 검증
5. provider가 반환하는 `QLPreviewReply` content type과 data 반환 방식 검토
6. 필요 시 서명/entitlement/UTI/provider 응답 수정

검증:
- Finder에서 `.hwp` 선택 후 스페이스바 Quick Look 표시 확인
- `pluginkit`에서 Preview/Thumbnail extension 등록 확인
- 실패 시 error/fallback plain text가 아닌 실제 페이지 미리보기 표시 확인

승인 필요:
- 2026-04-22 작업지시자의 "문제를 정의하고 해결" 요청을 Stage 2/3 진행 승인으로 간주

---

#### Stage 4 — 통합 회귀 검증 및 보고서

상태: 완료

작업:
1. HostApp Viewer, Thumbnail, Quick Look Preview를 같은 샘플 세트로 검증
2. Debug/Release 빌드 차이를 확인
3. Task #6 최종 보고서 작성
4. Stage 7 배포 이슈 #5 진행 가능 여부 정리

검증:
- `KTX.hwp` 첫 페이지 PNG 렌더 체크
- `KTX.hwp` Finder 아이콘 뷰 16/32/64 단계 Thumbnail
- `KTX.hwp` Finder Quick Look Preview
- 서명된 Debug 앱 설치 및 `pluginkit` 등록 확인

승인 필요:
- 2026-04-22 작업지시자가 현재까지 작업으로 Issue #6을 마무리하고 로컬 `macos/devel`에 PR 방식으로 반영하도록 요청

### 위험 요소

| 위험 | 대응 |
|------|------|
| Finder extension 캐시 때문에 수정 반영이 늦음 | `qlmanage`/`pluginkit`/앱 재설치 절차를 검증 로그에 명시 |
| Debug 무서명 빌드와 설치 빌드 동작 차이 | Finder 검증은 서명/설치된 앱 기준으로 별도 확인 |
| HostApp 문제와 extension 문제 혼동 | HostApp, Preview, Thumbnail 경로를 분리해 검증 |
| 기존 iOS 미커밋 변경 혼입 | 사용자가 Task #6에 포함하기로 한 변경과 macOS 변경을 최종 보고서에서 분리 표기 |

### 현재 절차 상태

Stage 1은 사용자 요청에 따라 이미 구현된 뒤 본 구현계획서로 소급 정리했다. Stage 2부터는 이 문서를 기준으로 승인 후 진행한다.
