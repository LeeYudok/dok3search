import { Check, Code, Copy, ListBullets } from '@phosphor-icons/react'
import { type ReactNode, useMemo, useState } from 'react'
import type { ReaderSectionItem } from './api'

export type SymbolOutlineItem = {
  id: string
  kind: 'fn' | 'class' | 'iface' | 'type' | 'const' | 'sym'
  name: string
  lineStart: number
  lineEnd: number
}

const SYMBOL_PREFIXES: Array<{ prefix: string; kind: SymbolOutlineItem['kind'] }> = [
  { prefix: 'function ', kind: 'fn' },
  { prefix: 'fn ', kind: 'fn' },
  { prefix: 'func ', kind: 'fn' },
  { prefix: 'def ', kind: 'fn' },
  { prefix: 'method ', kind: 'fn' },
  { prefix: 'class ', kind: 'class' },
  { prefix: 'record ', kind: 'class' },
  { prefix: 'struct ', kind: 'class' },
  { prefix: 'impl ', kind: 'class' },
  { prefix: 'interface ', kind: 'iface' },
  { prefix: 'trait ', kind: 'iface' },
  { prefix: 'type ', kind: 'type' },
  { prefix: 'enum ', kind: 'type' },
  { prefix: 'const ', kind: 'const' },
  { prefix: 'let ', kind: 'const' },
]

export function parseSymbolOutlineItem(sec: ReaderSectionItem): SymbolOutlineItem | null {
  if (sec.sectionId === 'body') return null
  if (sec.sectionId === 'preamble') {
    return {
      id: sec.sectionId,
      kind: 'sym',
      name: '파일 헤더',
      lineStart: sec.lineStart,
      lineEnd: sec.lineEnd,
    }
  }
  const heading = sec.heading.trim()
  for (const { prefix, kind } of SYMBOL_PREFIXES) {
    if (heading.startsWith(prefix)) {
      return {
        id: sec.sectionId,
        kind,
        name: heading.slice(prefix.length).trim() || heading,
        lineStart: sec.lineStart,
        lineEnd: sec.lineEnd,
      }
    }
  }
  return {
    id: sec.sectionId,
    kind: 'sym',
    name: heading,
    lineStart: sec.lineStart,
    lineEnd: sec.lineEnd,
  }
}

function detectLanguageLabel(samplePath?: string): string {
  if (!samplePath) return 'Code'
  const lower = samplePath.toLowerCase()
  if (lower.endsWith('.tsx')) return 'React TSX'
  if (lower.endsWith('.jsx')) return 'React JSX'
  if (/\.(ts|mts|cts)$/.test(lower)) return 'TypeScript'
  if (/\.(js|mjs|cjs)$/.test(lower)) return 'JavaScript'
  if (lower.endsWith('.rs')) return 'Rust'
  if (lower.endsWith('.go')) return 'Go'
  if (lower.endsWith('.py')) return 'Python'
  if (lower.endsWith('.java')) return 'Java'
  if (lower.endsWith('.sql')) return 'SQL'
  if (lower.endsWith('.jsp')) return 'JSP'
  return 'Text'
}

const KEYWORDS = new Set([
  'export', 'import', 'from', 'as', 'function', 'const', 'let', 'var', 'class', 'interface',
  'type', 'enum', 'async', 'await', 'return', 'if', 'else', 'for', 'while', 'switch', 'case',
  'break', 'continue', 'try', 'catch', 'finally', 'throw', 'new', 'typeof', 'instanceof', 'void',
  'null', 'undefined', 'true', 'false', 'declare', 'default', 'readonly', 'private', 'public',
  'protected', 'static', 'extends', 'implements', 'record', 'throws', 'synchronized', 'abstract',
  'final', 'pub', 'fn', 'struct', 'impl', 'trait', 'mod',
  'use', 'match', 'mut', 'where', 'def', 'func', 'package', 'SELECT', 'FROM', 'WHERE', 'CREATE',
  'TABLE', 'OR', 'REPLACE', 'VIEW', 'INDEX', 'INSERT', 'UPDATE', 'DELETE', 'JOIN',
])

const TOKEN_RE = /("(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\.|[^`\\])*`|@[A-Za-z_][A-Za-z0-9_]*|\b\d+(?:\.\d+)?\b|\b[A-Za-z_$][A-Za-z0-9_$]*\b)/g

function renderCodeSegment(segment: string, keyPrefix: string): ReactNode[] {
  const nodes: ReactNode[] = []
  let lastIndex = 0
  TOKEN_RE.lastIndex = 0
  let match: RegExpExecArray | null
  let idx = 0
  while ((match = TOKEN_RE.exec(segment)) !== null) {
    if (match.index > lastIndex) {
      nodes.push(segment.slice(lastIndex, match.index))
    }
    const token = match[0]
    const key = `${keyPrefix}-${idx++}`
    if (token.startsWith('"') || token.startsWith("'") || token.startsWith('`')) {
      nodes.push(<span className="tok-str" key={key}>{token}</span>)
    } else if (token.startsWith('@') || KEYWORDS.has(token)) {
      nodes.push(<span className="tok-kw" key={key}>{token}</span>)
    } else if (/^\d/.test(token)) {
      nodes.push(<span className="tok-num" key={key}>{token}</span>)
    } else if (/^[A-Z][A-Za-z0-9_]*$/.test(token)) {
      nodes.push(<span className="tok-type" key={key}>{token}</span>)
    } else {
      nodes.push(token)
    }
    lastIndex = match.index + token.length
  }
  if (lastIndex < segment.length) {
    nodes.push(segment.slice(lastIndex))
  }
  return nodes
}

function renderHighlightedLine(line: string): ReactNode {
  const trimmed = line.trimStart()
  if (
    trimmed.startsWith('//') ||
    trimmed.startsWith('/*') ||
    trimmed.startsWith('*') ||
    trimmed.startsWith('#') ||
    trimmed.startsWith('--') ||
    trimmed.startsWith('<!--') ||
    trimmed.startsWith('<%--')
  ) {
    return <span className="tok-comment">{line}</span>
  }
  const slashIdx = line.indexOf('//')
  if (slashIdx > 0 && !line.slice(0, slashIdx).includes('http:') && !line.slice(0, slashIdx).includes('https:')) {
    return (
      <>
        {renderCodeSegment(line.slice(0, slashIdx), 'c')}
        <span className="tok-comment">{line.slice(slashIdx)}</span>
      </>
    )
  }
  return renderCodeSegment(line, 'l')
}

export function CodeDocumentViewer({
  body,
  samplePath,
  sections,
  activeAnchor,
  markdownFallbackRender,
}: {
  body: string
  samplePath?: string
  sections: readonly ReaderSectionItem[]
  activeAnchor: string
  markdownFallbackRender: () => ReactNode
}) {
  const [viewMode, setViewMode] = useState<'file' | 'cards'>('file')
  const [copied, setCopied] = useState(false)

  const lines = useMemo(() => body.replace(/\r\n/g, '\n').split('\n'), [body])
  const langLabel = detectLanguageLabel(samplePath)

  const sectionByLine = useMemo(() => {
    const map = new Map<number, ReaderSectionItem>()
    for (const sec of sections) {
      if (sec.lineStart > 0 && !map.has(sec.lineStart)) {
        map.set(sec.lineStart, sec)
      }
    }
    return map
  }, [sections])

  const activeRange = useMemo(() => {
    const sec = sections.find((s) => s.sectionId === activeAnchor)
    return sec ? { start: sec.lineStart, end: sec.lineEnd } : null
  }, [sections, activeAnchor])

  async function copyAll() {
    try {
      await navigator.clipboard.writeText(body)
      setCopied(true)
      window.setTimeout(() => setCopied(false), 1600)
    } catch {
      setCopied(false)
    }
  }

  return (
    <div className="code-viewer">
      <div className="code-toolbar">
        <span className="code-toolbar-meta">
          <Code size={15} aria-hidden="true" />
          <span>{langLabel}</span>
          <span>· {lines.length}줄</span>
          {sections.length > 1 && <span>· 심볼 {sections.length}개</span>}
        </span>
        <div className="code-toolbar-actions">
          <div className="code-view-toggle" role="group" aria-label="코드 보기 방식">
            <button
              type="button"
              aria-pressed={viewMode === 'file'}
              onClick={() => setViewMode('file')}
            >
              <Code size={14} aria-hidden="true" />
              파일 전체
            </button>
            <button
              type="button"
              aria-pressed={viewMode === 'cards'}
              onClick={() => setViewMode('cards')}
            >
              <ListBullets size={14} aria-hidden="true" />
              심볼 카드
            </button>
          </div>
          <button type="button" className="code-copy-btn" onClick={() => void copyAll()}>
            {copied ? <Check size={14} weight="bold" aria-hidden="true" /> : <Copy size={14} aria-hidden="true" />}
            {copied ? '복사됨' : '전체 복사'}
          </button>
        </div>
      </div>

      {viewMode === 'cards' ? (
        markdownFallbackRender()
      ) : (
        <div className="code-scroll" tabIndex={0}>
          <table className="code-table">
            <tbody>
              {lines.map((line, idx) => {
                const lineNo = idx + 1
                const sym = sectionByLine.get(lineNo)
                const isActive = Boolean(activeRange && lineNo >= activeRange.start && lineNo <= activeRange.end)
                return (
                  <tr
                    key={lineNo}
                    id={sym?.sectionId}
                    data-symbol-anchor={sym?.sectionId}
                    className={isActive ? 'code-row code-row-active' : 'code-row'}
                  >
                    <td className="code-ln" id={`L${lineNo}`}>
                      <a href={`#L${lineNo}`}>{lineNo}</a>
                    </td>
                    <td className="code-line">
                      <code>{line.length > 0 ? renderHighlightedLine(line) : ' '}</code>
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
