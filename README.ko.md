# Highjump

[🇺🇸 Read in English](README.md)

Highjump는 디렉토리를 북마크하고 원활하게 이동할 수 있도록 도와주는 빠르고 직관적인 CLI 도구입니다. 자주 사용하는 경로로 빠르게 이동할 수 있는 대화형 퍼지(fuzzy) 검색 인터페이스를 제공합니다.

## 주요 기능

- **현재 디렉토리 북마크:** 현재 작업 중인 디렉토리를 쉽게 저장합니다.
- **대화형 퍼지 검색:** 방향키, 경로명 타이핑 또는 인덱스 번호 선택을 통해 저장된 디렉토리를 찾고 즉시 이동합니다.
- **영구 저장:** 저장된 경로는 `~/.highjump_paths.json` 파일에 안전하게 보관됩니다.

## 설치 방법

### 1. Rust CLI 빌드

먼저, 저장소를 클론하고 Cargo를 사용하여 프로젝트를 빌드합니다:

```bash
git clone https://github.com/hooneun/highjump
cd highjump
cargo build --release
```

컴파일된 바이너리를 시스템의 PATH가 지정된 디렉토리로 이동시킵니다:

```
cp target/release/highjump ~/.cargo/bin/highjump
```

(참고: ~/.cargo/bin 또는 선택한 디렉토리가 시스템의 $PATH에 포함되어 있어야 합니다.)

### 2. 쉘 래퍼(Shell Wrapper) 설정

UNIX 기반 운영체제의 근본적인 구조상, 자식 프로세스(Rust CLI)는 부모 프로세스(Shell)의 현재 작업 디렉토리를 직접 변경할 수 없습니다.

cd 기능을 활성화하려면 반드시 쉘 함수를 추가해야 합니다. 다음 코드를 ~/.zshrc 또는 ~/.bashrc 파일 맨 아래에 추가하세요:

```bash
# Highjump shell wrapper
function hj() {
    if [ $# -eq 0 ]; then
        # Navigation mode
        local TARGET_DIR=$(highjump)
        if [ -n "$TARGET_DIR" ] && [ -d "$TARGET_DIR" ]; then
            cd "$TARGET_DIR" || return
        fi
    else
        # Save mode or Help (--save, --help)
        highjump "$@"
    fi
}
```

쉘 설정을 다시 불러옵니다:

```bash
source ~/.zshrc  # 또는 source ~/.bashrc
```
## 사용 방법

모든 작업에 hj 단축 명령어를 사용합니다.

**현재 디렉토리 저장:**

```bash
hj --save
# 또는
hj -s
```

**저장된 디렉토리로 이동:**

```bash
hj
```

대화형 프롬프트가 열리면 다음 작업을 수행할 수 있습니다:

* **위/아래 방향키**를 사용하여 탐색합니다.
* **번호**를 입력하여 번호로 필터링합니다.
* **디렉토리 경로의 일부를 타이핑**하여 검색을 수행합니다.
* **Enter**를 눌러 해당 위치로 이동합니다.

도움말 보기:

```bash
hj --help
```