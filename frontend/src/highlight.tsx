import type { ReactNode } from 'react'
import { expandSearchAliases } from './documents'

const HANGUL_PARTICLES = ['에서', '으로', '은', '는', '을', '를', '과', '와', '의'] as const
const MAX_HIGHLIGHT_TERMS = 12

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

function stripQuotes(token: string): string {
  if (
    (token.startsWith('"') && token.endsWith('"') && token.length >= 2) ||
    (token.startsWith("'") && token.endsWith("'") && token.length >= 2)
  ) {
    return token.slice(1, -1).trim()
  }
  return token
}

function tokenizeQuery(query: string): string[] {
  const tokens: string[] = []
  const re = /"[^"]*"|'[^']*'|\S+/g
  let match: RegExpExecArray | null
  while ((match = re.exec(query)) !== null) {
    if (match[0]) tokens.push(match[0])
  }
  return tokens
}

export function extractHighlightTerms(query: string, topicOrCategory?: string): string[] {
  const trimmed = query.trim()
  if (!trimmed) return []

  const candidates: string[] = []
  const plainTokens: string[] = []

  for (const rawToken of tokenizeQuery(trimmed)) {
    const colonIdx = rawToken.indexOf(':')
    if (colonIdx > 0) {
      const op = rawToken.slice(0, colonIdx).toLowerCase()
      const val = stripQuotes(rawToken.slice(colonIdx + 1).trim())
      if (op === 'sym' || op === 'symbol') {
        if (val) candidates.push(val)
        continue
      }
      if (op === 'path') {
        if (val) {
          candidates.push(val)
          for (const seg of val.split('/')) {
            if (seg.length >= 2) candidates.push(seg)
          }
        }
        continue
      }
      if (op === 'ext') {
        const cleanExt = val.replace(/^\.+/, '')
        if (cleanExt) candidates.push(`.${cleanExt}`)
        continue
      }
    }

    const plain = stripQuotes(rawToken)
    if (!plain) continue
    plainTokens.push(plain)
    candidates.push(plain)

    if (/^[가-힣]{3,}$/.test(plain)) {
      for (const particle of HANGUL_PARTICLES) {
        if (plain.endsWith(particle) && plain.length - particle.length >= 2) {
          candidates.push(plain.slice(0, -particle.length))
          break
        }
      }
    }

    for (const alias of expandSearchAliases(plain.toLowerCase(), topicOrCategory)) {
      candidates.push(alias)
    }
  }

  if (plainTokens.length >= 2) {
    candidates.unshift(plainTokens.join(' '))
  }

  const seen = new Set<string>()
  const unique: string[] = []
  for (const term of candidates) {
    const clean = term.trim()
    if (!clean) continue
    const key = clean.toLowerCase()
    if (seen.has(key)) continue
    seen.add(key)
    unique.push(clean)
  }

  unique.sort((a, b) => b.length - a.length)
  return unique.slice(0, MAX_HIGHLIGHT_TERMS)
}

function buildHighlightRegex(query: string, topicOrCategory?: string): RegExp | null {
  const terms = extractHighlightTerms(query, topicOrCategory)
  if (terms.length === 0) return null
  return new RegExp(`(${terms.map(escapeRegExp).join('|')})`, 'gi')
}

export function highlightText(
  text: string,
  query: string,
  topicOrCategory?: string,
): ReactNode {
  if (!text || !query.trim()) return text
  const regex = buildHighlightRegex(query, topicOrCategory)
  if (!regex) return text

  const nodes: ReactNode[] = []
  let lastIndex = 0
  let idx = 0
  let match: RegExpExecArray | null
  while ((match = regex.exec(text)) !== null) {
    if (match[0].length === 0) break
    if (match.index > lastIndex) {
      nodes.push(text.slice(lastIndex, match.index))
    }
    nodes.push(
      <mark className="search-hl" key={idx++}>
        {match[0]}
      </mark>,
    )
    lastIndex = match.index + match[0].length
  }

  if (nodes.length === 0) return text
  if (lastIndex < text.length) {
    nodes.push(text.slice(lastIndex))
  }
  return nodes
}

export function buildContextSnippet(
  summary: string,
  body: string,
  query: string,
  topicOrCategory?: string,
  maxChars = 180,
): string {
  const fallback = summary || body.slice(0, 200)
  if (!query.trim()) return fallback

  const regex = buildHighlightRegex(query, topicOrCategory)
  if (!regex) return fallback

  if (summary && regex.test(summary)) {
    return summary
  }

  const clean = body.replace(/\s+/g, ' ').trim()
  if (!clean) return fallback

  regex.lastIndex = 0
  const match = regex.exec(clean)
  if (!match) return fallback

  let start = Math.max(0, match.index - 48)
  if (start > 0) {
    const nextSpace = clean.indexOf(' ', start)
    if (nextSpace !== -1 && nextSpace < match.index) {
      start = nextSpace + 1
    }
  }
  const slice = clean.slice(start, start + maxChars).trimEnd()
  return start > 0 ? `…${slice}` : slice
}
