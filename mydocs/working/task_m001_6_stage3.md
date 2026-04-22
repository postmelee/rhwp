# Task #6 — Stage 3 완료보고서

## Quick Look Preview 미동작 원인 분리 및 복구

### 메타데이터

| 항목 | 값 |
|------|-----|
| 이슈 | `postmelee/rhwp#6` |
| 마일스톤 | `M001` — macOS Quick Look v0.1 안정화 |
| 작업 브랜치 | `local/task6` |
| 기준 브랜치 | `macos/devel` |
| 단계 | Stage 3 |
| 작성일 | 2026-04-22 |

### 문제 정의

Quick Look이 동작하지 않는 직접 원인은 provider 구현보다 설치/등록 상태에 있었다. 확인 시 `/Applications/HWP Quick Look.app`이 없었고, 기존 `rhwp-macos/build/debug/HWP Quick Look.app`은 `CODE_SIGNING_ALLOWED=NO`로 만든 검증용 산출물이라 Finder/Quick Look extension 등록 대상으로 신뢰하기 어려웠다.

즉 Finder 통합 검증은 단순 빌드 산출물이 아니라 서명된 앱 번들을 `/Applications`에 설치하고 LaunchServices/Quick Look에 등록한 상태에서 수행해야 한다.

### 수행 내용

1. `HwpPreviewProvider.swift`의 framework import를 검토했고, 현재 SDK에서는 기존 `QuickLookUI` import가 맞음을 확인했다.
2. 서명된 Debug 앱을 빌드하고 `/Applications/HWP Quick Look.app`에 설치하는 `install-debug-app.sh`를 추가했다.
3. 설치 스크립트에서 bundle signature 검증, LaunchServices 등록, `qlmanage -r`, `pluginkit` 등록 확인을 한 번에 수행하도록 했다.
4. Finder에서 `KTX.hwp`를 선택하고 스페이스바 Quick Look을 실행해 첫 페이지 preview 표시를 확인했다.

### 변경 파일

| 파일 | 변경 |
|------|------|
| `rhwp-macos/scripts/install-debug-app.sh` | 서명 빌드, `/Applications` 설치, Quick Look 등록 갱신 스크립트 추가 |

### 검증 결과

| 항목 | 결과 |
|------|------|
| 서명된 Debug 앱 빌드 | 통과 |
| `codesign --verify --deep --strict` | 통과 |
| `/Applications/HWP Quick Look.app` 설치 | 통과 |
| `pluginkit` Preview extension 등록 | 통과 (`com.postmelee.rhwpmac.QLExtension`) |
| `pluginkit` Thumbnail extension 등록 | 통과 (`com.postmelee.rhwpmac.ThumbnailExtension`) |
| Finder 스페이스바 Quick Look | 통과 (`KTX.hwp` 첫 페이지 표시) |

### 실행 명령

```bash
rhwp-macos/scripts/install-debug-app.sh

/usr/bin/open -R \
  /Users/melee/Documents/projects/rhwp-macos/samples/basic/KTX.hwp
```

### 특이사항

- `qlmanage -p`/`qlmanage -t` 계열 CLI는 macOS 26.4.1 환경에서 실제 Finder UI와 다르게 멈추거나 예외를 낸 이력이 있다. 이번 검증도 사용자-facing 기준인 Finder Quick Look UI와 QuickLookThumbnailing API를 기준으로 삼았다.
- `mdls` 기준 `KTX.hwp`의 UTI는 `com.haansoft.hancomofficeviewer.mac.hwp`이고, HostApp/extension의 supported content type에 포함되어 있다.
