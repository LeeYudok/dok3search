import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { CodeDocumentViewer, parseSymbolOutlineItem } from '../code-viewer'

describe('parseSymbolOutlineItem', () => {
  it('maps symbol prefixes to compact kinds and handles preamble/body', () => {
    expect(parseSymbolOutlineItem({ sectionId: 'body', heading: 'x', anchor: 'body', lineStart: 1, lineEnd: 1 })).toBeNull()
    expect(parseSymbolOutlineItem({ sectionId: 'preamble', heading: 'utils.ts', anchor: 'preamble', lineStart: 1, lineEnd: 4 })?.name).toBe('파일 헤더')
    expect(parseSymbolOutlineItem({ sectionId: 'fn-1', heading: 'function toArrayBuffer', anchor: 'fn-1', lineStart: 9, lineEnd: 20 })).toMatchObject({
      kind: 'fn',
      name: 'toArrayBuffer',
      lineStart: 9,
    })
    expect(parseSymbolOutlineItem({ sectionId: 'cls-1', heading: 'class KordocError', anchor: 'cls-1', lineStart: 21, lineEnd: 31 })?.kind).toBe('class')
    expect(parseSymbolOutlineItem({ sectionId: 'if-1', heading: 'interface Opts', anchor: 'if-1', lineStart: 32, lineEnd: 35 })?.kind).toBe('iface')
    expect(parseSymbolOutlineItem({ sectionId: 'tp-1', heading: 'type Mode', anchor: 'tp-1', lineStart: 36, lineEnd: 36 })?.kind).toBe('type')
    expect(parseSymbolOutlineItem({ sectionId: 'cn-1', heading: 'const VERSION', anchor: 'cn-1', lineStart: 7, lineEnd: 8 })?.kind).toBe('const')
  })
})

describe('CodeDocumentViewer', () => {
  const sampleBody = [
    '// leading comment',
    'export const VERSION = "1.0" // inline',
    'export function toArrayBuffer(n: number): number {',
    '  return n + 42',
    '}',
  ].join('\n')

  it('renders line numbers, syntax tokens, active symbol highlight, view toggle, and copy button', async () => {
    const writeText = vi.fn().mockResolvedValue(undefined)
    Object.assign(navigator, { clipboard: { writeText } })

    render(
      <CodeDocumentViewer
        body={sampleBody}
        samplePath="samples/kordoc/src/utils.ts"
        sections={[
          { sectionId: 'const-version', heading: 'const VERSION', anchor: 'const-version', lineStart: 2, lineEnd: 2 },
          { sectionId: 'function-toarraybuffer', heading: 'function toArrayBuffer', anchor: 'function-toarraybuffer', lineStart: 3, lineEnd: 5 },
        ]}
        activeAnchor="function-toarraybuffer"
        markdownFallbackRender={() => <div data-testid="cards-fallback">cards</div>}
      />,
    )

    expect(screen.getByText('TypeScript')).toBeInTheDocument()
    expect(screen.getByText('· 5줄')).toBeInTheDocument()
    expect(document.querySelector('.tok-comment')).not.toBeNull()
    expect(document.querySelector('.tok-kw')).not.toBeNull()
    expect(document.querySelector('.tok-str')).not.toBeNull()
    expect(document.querySelector('.tok-num')).not.toBeNull()
    expect(document.querySelectorAll('.code-row-active').length).toBe(3)

    fireEvent.click(screen.getByRole('button', { name: '전체 복사' }))
    expect(writeText).toHaveBeenCalledWith(sampleBody)

    fireEvent.click(screen.getByRole('button', { name: '심볼 카드' }))
    expect(screen.getByTestId('cards-fallback')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: '파일 전체' }))
    expect(document.querySelector('.code-table')).not.toBeNull()
  })

  it('detects React TSX, JSP, and Java annotations/records cleanly', () => {
    const { rerender } = render(
      <CodeDocumentViewer
        body={'export const OrderBadge = () => <span>OK</span>'}
        samplePath="samples/polyglot-samples/react/OrderStatusBadge.tsx"
        sections={[]}
        activeAnchor=""
        markdownFallbackRender={() => null}
      />,
    )
    expect(screen.getByText('React TSX')).toBeInTheDocument()

    rerender(
      <CodeDocumentViewer
        body={'<%-- JSP comment --%>\n<%@ page contentType="text/html; charset=UTF-8" %>'}
        samplePath="samples/polyglot-samples/jsp/orderList.jsp"
        sections={[]}
        activeAnchor=""
        markdownFallbackRender={() => null}
      />,
    )
    expect(screen.getByText('JSP')).toBeInTheDocument()
    expect(document.querySelector('.tok-comment')).not.toBeNull()

    expect(
      parseSymbolOutlineItem({
        sectionId: 'record-order',
        heading: 'record OrderCreateRequest',
        anchor: 'record-order',
        lineStart: 5,
        lineEnd: 9,
      })?.kind,
    ).toBe('class')

    rerender(
      <CodeDocumentViewer
        body={'@RestController\npublic record OrderCreateRequest(String orderId) {}'}
        samplePath="samples/polyglot-samples/java/OrderController.java"
        sections={[]}
        activeAnchor=""
        markdownFallbackRender={() => null}
      />,
    )
    expect(screen.getByText('Java')).toBeInTheDocument()
    expect(screen.getByText('@RestController')).toHaveClass('tok-kw')
  })
})
