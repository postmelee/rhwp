# Task #6 — 수행계획서

## macOS Quick Look: 첫 페이지 렌더링 누락 및 Finder 통합 안정화

### 메타데이터

| 항목 | 값 |
|------|-----|
| 이슈 | `postmelee/rhwp#6` |
| 마일스톤 | `M001` — macOS Quick Look v0.1 안정화 |
| 기준 브랜치 | `macos/devel` |
| 작업 브랜치 | `local/task6` |
| 관련 선행 작업 | `postmelee/rhwp#3` macOS Quick Look Viewer |
| 현재 상태 | Stage 1~4 완료, 로컬 `macos/devel` merge 준비 |

### 배경

`local/task3` 작업분을 `macos/devel`에 로컬 merge하고 fork 원격 `origin/macos/devel`까지 push한 뒤, `HWP Quick Look.app`에서 `.hwp` 문서를 열 때 첫 페이지가 보이지 않는 문제가 확인되었다. 이후 확인 과정에서 Finder 아이콘 뷰의 특정 줌 단계에서 Thumbnail이 보이지 않거나, Quick Look Preview 확장이 동작하지 않는 문제도 별도로 확인되었다.

Task #6은 macOS HostApp Viewer와 Finder Quick Look/Thumbnail 통합이 사용자 관점에서 안정적으로 동작하도록 만드는 안정화 타스크다.

### 문제 정의

1. HostApp Viewer 첫 페이지 누락
   - 문서명과 페이지 수는 표시되지만 첫 페이지 영역이 흰 화면 또는 로딩 상태로 남는다.
   - 일부 문서에서는 두 번째 페이지도 늦게 보이거나 비어 보일 수 있다.

2. Finder Thumbnail 불안정
   - Finder 아이콘 뷰의 특정 줌 단계에서 썸네일이 보이지 않는다.
   - 너무 작거나 너무 커서 보이지 않는 단순 스케일 문제로 보이지 않으며, 줌 조정에 따라 보임/안 보임이 바뀐다.

3. Quick Look Preview 미동작
   - HostApp Viewer와 별개로 Finder 스페이스바 Quick Look 경로가 현재 동작하지 않는다.
   - 원인은 확장 등록, 서명, UTI 매칭, provider 응답 형식 중 하나일 수 있다.

### 목표

- HostApp에서 HWP/HWPX 문서 첫 페이지가 즉시 표시되도록 한다.
- 다중 페이지 문서에서 초기 페이지와 인접 페이지가 안정적으로 로드되도록 한다.
- Finder Thumbnail의 줌 단계별 표시 불안정 원인을 정의하고 수정한다.
- Finder Quick Look Preview가 실제 Finder 경로에서 동작하도록 복구한다.
- 각 단계별 검증 결과를 보고서로 남긴다.

### 범위

포함:
- HostApp Viewer 초기 페이지 선로드/캐시 정책 수정
- HostApp 레이아웃 단순화 및 사이드바 제거
- Finder Thumbnail provider 표시 불안정 조사 및 수정
- Quick Look Preview provider/extension 등록 경로 조사 및 수정
- Debug 빌드, PNG 렌더 체크, 실제 앱/Finder 검증

제외:
- Stage 7 배포 패키징, GitHub Release, Homebrew cask
- 편집/저장 기능
- upstream iOS Viewer 기능 범위를 초과하는 macOS 전용 편집 기능

### 산출물

| 파일 | 내용 |
|------|------|
| `mydocs/orders/20260422.md` | 오늘 할일 및 Task #6 진행 상태 |
| `mydocs/plans/task_m001_6.md` | 수행계획서 |
| `mydocs/plans/task_m001_6_impl.md` | 구현계획서 |
| `mydocs/working/task_m001_6_stage*.md` | 단계별 완료보고서 |
| `rhwp-macos/Sources/HostApp/...` | HostApp Viewer 안정화 |
| `rhwp-macos/Sources/QLExtension/...` | Quick Look Preview 안정화 예정 |
| `rhwp-macos/Sources/ThumbnailExtension/...` | Finder Thumbnail 안정화 예정 |

### 승인 상태

이 문서는 이미 수행된 Stage 1 변경분을 AGENTS.md 절차에 맞춰 소급 정리하기 위해 작성했다. Stage 2/3은 2026-04-22 작업지시자의 "문제를 정의하고 해결" 요청을 승인으로 간주해 진행했다. 2026-04-22 작업지시자가 현재까지 작업으로 Issue #6을 마무리하고 로컬 `macos/devel`에 PR 방식으로 반영하도록 요청했다.
