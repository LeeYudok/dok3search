import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { AdminPage } from '../admin-page'
import * as api from '../api'

describe('AdminPage', () => {
  it('renders sources, opens add-source form, and triggers analyzer', async () => {
    vi.spyOn(api, 'fetchSources').mockResolvedValue({
      sources: [
        {
          sourceId: 'local-markdown',
          label: 'Local Markdown',
          path: '/app/knowledge',
          kind: 'local_dir',
          defaultCategory: 'guide',
          defaultTag: 'docs',
          managedByEnv: true,
          createdAtMs: 1000,
          documentCount: 2035,
        },
      ],
    })
    vi.spyOn(api, 'fetchIndexJobs').mockResolvedValue({
      jobs: [],
      activeGeneration: 'gen-1',
      activeEpoch: 1,
    })
    vi.spyOn(api, 'analyzeSearchText').mockResolvedValue({
      profile: 'nori-mixed',
      analyzerVersion: 1,
      normalizedQueryTerms: ['테스트'],
      terms: ['테스트'],
      charNgrams: ['테스', '스트'],
    })

    render(<AdminPage initialTab="sources" />)
    await waitFor(() => {
      expect(screen.getByText('Local Markdown')).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: '새 소스 등록' }))
    expect(screen.getByLabelText(/소스 ID/)).toBeInTheDocument()

    fireEvent.click(screen.getByRole('link', { name: '분석기 실험' }))
    fireEvent.click(screen.getByRole('button', { name: '분석 실행' }))
    await waitFor(() => {
      expect(screen.getByText('nori-mixed')).toBeInTheDocument()
    })
  })

  it('shows 403 forbidden alert when admin API is disabled', async () => {
    vi.spyOn(api, 'fetchSources').mockRejectedValue(new api.ApiRequestError('forbidden', 403, 'FORBIDDEN'))
    vi.spyOn(api, 'fetchIndexJobs').mockRejectedValue(new api.ApiRequestError('forbidden', 403, 'FORBIDDEN'))

    render(<AdminPage initialTab="jobs" />)
    await waitFor(() => {
      expect(screen.getByRole('alert')).toHaveTextContent('DOK3_ENABLE_ADMIN=true')
    })
  })
})
