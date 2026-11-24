# kordoc 소스 코퍼스 (HWP·HWPX·PDF → Markdown 파서)

- 출처: `https://github.com/LeeYudok/kordoc` (`v4.18.8`, 커밋 `25f385d71a4a`)
- 라이선스: MIT
- 범위: `src/**` TypeScript 구현 소스만 반입 (`tests/`, `bench/`, 바이너리 픽스처 제외)

재생성 명령:

```bash
python3 scripts/import-kordoc-source.py --source <kordoc-dir> --output frontend/src/samples/kordoc
```
