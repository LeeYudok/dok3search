import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { buildContextSnippet, extractHighlightTerms, highlightText } from '../highlight'

describe('extractHighlightTerms', () => {
  it('extracts operator values without literal prefixes and handles Hangul stems + aliases', () => {
    expect(extractHighlightTerms('path:api/orders ext:ts sym:POST')).toEqual([
      'api/orders',
      'orders',
      'POST',
      'api',
      '.ts',
    ])
    expect(extractHighlightTerms('소유권을')).toEqual(['소유권을', '소유권'])
    expect(extractHighlightTerms('세대 전환')).toEqual(['세대 전환', '세대', '전환'])
    expect(extractHighlightTerms('고루틴', 'go')).toEqual(['goroutine', '고루틴'])
  })
})

describe('highlightText', () => {
  it('wraps case-insensitive matches in <mark class="search-hl"> and escapes regex chars', () => {
    const { container } = render(
      <span>{highlightText('export function toArrayBuffer(buf: Buffer)', 'toarraybuffer')}</span>,
    )
    const marks = container.querySelectorAll('mark.search-hl')
    expect(marks).toHaveLength(1)
    expect(marks[0].textContent).toBe('toArrayBuffer')

    const { container: metaContainer } = render(
      <span>{highlightText('fn parse() -> Result<Vec<u8>> and c:forEach ($order)', 'Result<Vec<u8>> $order')}</span>,
    )
    const metaMarks = [...metaContainer.querySelectorAll('mark.search-hl')].map((m) => m.textContent)
    expect(metaMarks).toEqual(['Result<Vec<u8>>', '$order'])
  })
})

describe('buildContextSnippet', () => {
  it('extracts a context window around the matched body keyword when summary lacks it', () => {
    const summary = '주문 정산 도메인 설명입니다.'
    const body = '앞부분 일반 설명이 길게 이어집니다. '.repeat(4) + '레거시 DB에서 ORA-00904 오류가 발생하면 폴백 쿼리를 실행합니다.'
    const snippet = buildContextSnippet(summary, body, 'ORA-00904', 'code')
    expect(snippet).toContain('ORA-00904')
    expect(snippet.startsWith('…')).toBe(true)

    expect(buildContextSnippet(summary, body, '주문 정산', 'code')).toBe(summary)
  })
})
