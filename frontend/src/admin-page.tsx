import {
  ArrowLeft,
  ArrowsClockwise,
  CheckCircle,
  Database,
  FolderPlus,
  LockSimple,
  Play,
  Trash,
  WarningCircle,
  X,
} from '@phosphor-icons/react'
import { type FormEvent, useEffect, useState } from 'react'
import {
  ApiRequestError,
  createSource,
  deleteSource,
  fetchIndexJobs,
  fetchSources,
  type IndexJobListResponse,
  type IndexJobMode,
  type RegisteredSource,
  type SourceKind,
  type TombstoneMode,
  type SqliteCursorMode,
  analyzeSearchText,
  type AnalyzeSearchResponse,
  triggerIndexJob,
} from './api'
import { categories, categoryLabels } from './documents'

export type AdminTab = 'sources' | 'jobs' | 'analyzers'

function formatTimestamp(ms?: number): string {
  if (!ms) return '동기화 이력 없음'
  return new Date(ms).toLocaleString('ko-KR')
}

function formatAdminError(err: unknown): { kind: 'forbidden' | 'unwired' | 'error'; message: string } {
  if (err instanceof ApiRequestError) {
    if (err.status === 403) {
      return {
        kind: 'forbidden',
        message: '관리자 기능이 비활성화되어 있습니다. DOK3_ENABLE_ADMIN=true 환경 변수로 서버를 실행하세요.',
      }
    }
    if (err.status === 400 && err.message.includes('persistent mode')) {
      return {
        kind: 'unwired',
        message: '영속 저장소 모드가 아닙니다. DOK3_SOURCE_DIR 과 DOK3_DATA_DIR 를 함께 설정해야 소스 및 색인 작업을 관리할 수 있습니다.',
      }
    }
    return { kind: 'error', message: err.message }
  }
  return {
    kind: 'error',
    message: err instanceof Error ? err.message : '요청을 처리하지 못했습니다.',
  }
}

export function AdminPage({ initialTab = 'sources' }: { initialTab?: AdminTab }) {
  const [tab, setTab] = useState<AdminTab>(initialTab)
  const [sources, setSources] = useState<RegisteredSource[]>([])
  const [jobsData, setJobsData] = useState<IndexJobListResponse | null>(null)
  const [loading, setLoading] = useState(true)
  const [accessBanner, setAccessBanner] = useState<{ kind: 'forbidden' | 'unwired' | 'error'; message: string } | null>(null)
  const [actionNotice, setActionNotice] = useState<string | null>(null)
  const [actionError, setActionError] = useState<string | null>(null)
  const [busy, setBusy] = useState(false)

  const [showAddForm, setShowAddForm] = useState(false)
  const [newSourceId, setNewSourceId] = useState('')
  const [newLabel, setNewLabel] = useState('')
  const [newPath, setNewPath] = useState('')
  const [newCategory, setNewCategory] = useState('guide')
  const [newTag, setNewTag] = useState('docs')
  const [newKind, setNewKind] = useState<SourceKind>('local_dir')
  const [sqSelect, setSqSelect] = useState('SELECT id, title, body FROM notes')
  const [sqId, setSqId] = useState('id')
  const [sqTitle, setSqTitle] = useState('title')
  const [sqBody, setSqBody] = useState('body')
  const [sqSummary, setSqSummary] = useState('')
  const [sqCursorMode, setSqCursorMode] = useState<SqliteCursorMode>('integer_pk')
  const [sqCursorColumn, setSqCursorColumn] = useState('id')
  const [confirmDeleteId, setConfirmDeleteId] = useState<string | null>(null)
  const [analyzeInput, setAnalyzeInput] = useState('설계검토및보완결과 getAccountList ORA-00904')
  const [analyzeSplit, setAnalyzeSplit] = useState(true)
  const [analyzeNgrams, setAnalyzeNgrams] = useState(true)
  const [analyzeResult, setAnalyzeResult] = useState<AnalyzeSearchResponse | null>(null)

  async function loadAll(signal?: AbortSignal) {
    setLoading(true)
    setAccessBanner(null)
    try {
      const [srcResp, jobsResp] = await Promise.all([
        fetchSources(signal),
        fetchIndexJobs(signal),
      ])
      if (signal?.aborted) return
      setSources(srcResp.sources)
      setJobsData(jobsResp)
    } catch (err) {
      if (signal?.aborted) return
      if (err instanceof DOMException && err.name === 'AbortError') return
      setAccessBanner(formatAdminError(err))
    } finally {
      if (!signal?.aborted) setLoading(false)
    }
  }

  useEffect(() => {
    document.title = (tab === 'sources' ? '소스 관리' : tab === 'jobs' ? '색인 작업' : '분석기 실험') + ' · dok3search'
  }, [tab])

  useEffect(() => {
    function handlePopState() {
      setTab(window.location.pathname.startsWith('/admin/jobs') ? 'jobs' : window.location.pathname.startsWith('/admin/analyzers') ? 'analyzers' : 'sources')
    }
    window.addEventListener('popstate', handlePopState)
    return () => window.removeEventListener('popstate', handlePopState)
  }, [])

  useEffect(() => {
    const controller = new AbortController()
    void loadAll(controller.signal)
    return () => controller.abort()
  }, [])

  function switchTab(next: AdminTab) {
    setTab(next)
    setActionError(null)
    const targetPath = next === 'jobs' ? '/admin/jobs' : next === 'analyzers' ? '/admin/analyzers' : '/admin/sources'
    if (window.location.pathname !== targetPath) {
      window.history.pushState(null, '', targetPath)
    }
  }

  async function handleCreateSource(event: FormEvent) {
    event.preventDefault()
    setBusy(true)
    setActionError(null)
    setActionNotice(null)
    try {
      const created = await createSource({
        sourceId: newSourceId,
        label: newLabel,
        path: newPath,
        kind: newKind,
        sqlite:
          newKind === 'sqlite'
            ? {
                selectQuery: sqSelect,
                idColumn: sqId,
                titleColumn: sqTitle,
                bodyColumn: sqBody,
                summaryColumn: sqSummary.trim() || undefined,
                cursorMode: sqCursorMode,
                cursorColumn: sqCursorColumn,
              }
            : undefined,
        defaultCategory: newCategory,
        defaultTag: newTag,
      })
      setNewSourceId('')
      setNewLabel('')
      setNewPath('')
      setNewCategory('guide')
      setNewTag('docs')
      setNewKind('local_dir')
      setShowAddForm(false)
      setActionNotice(`소스 '${created.sourceId}' 를 등록하고 문서 ${created.documentCount}건을 색인했습니다.`)
      await loadAll()
    } catch (err) {
      setActionError(formatAdminError(err).message)
    } finally {
      setBusy(false)
    }
  }

  async function handleDeleteSource(sourceId: string) {
    setBusy(true)
    setActionError(null)
    setActionNotice(null)
    try {
      const result = await deleteSource(sourceId)
      setConfirmDeleteId(null)
      setActionNotice(
        `소스 '${result.sourceId}' 등록을 해제하고 색인 문서 ${result.deletedDocuments}건을 제거했습니다. 디스크 원본 파일은 그대로 유지됩니다.`,
      )
      await loadAll()
    } catch (err) {
      setActionError(formatAdminError(err).message)
    } finally {
      setBusy(false)
    }
  }

  async function handleAnalyze(event: FormEvent) {
    event.preventDefault()
    setBusy(true)
    setActionError(null)
    try {
      setAnalyzeResult(
        await analyzeSearchText({
          text: analyzeInput,
          splitIdentifiers: analyzeSplit,
          charNgrams: analyzeNgrams,
        }),
      )
    } catch (err) {
      setActionError(formatAdminError(err).message)
    } finally {
      setBusy(false)
    }
  }

  async function handleTriggerJob(mode: IndexJobMode, sourceId?: string) {
    setBusy(true)
    setActionError(null)
    setActionNotice(null)
    try {
      const job = await triggerIndexJob({ sourceId, mode })
      if (job.state === 'failed') {
        setActionError(`작업 ${job.jobId} 실패: ${job.error ?? '알 수 없는 오류'}`)
      } else {
        const targetLabel = job.sourceId ? `소스 '${job.sourceId}'` : '전체 소스'
        setActionNotice(
          `${targetLabel} ${job.mode === 'full' ? '전체 재색인' : '증분 새로고침'} 완료 (${job.jobId}, epoch ${job.activeEpoch}, ${job.indexedDocuments}건).`,
        )
      }
      await loadAll()
    } catch (err) {
      setActionError(formatAdminError(err).message)
    } finally {
      setBusy(false)
    }
  }

  return (
    <>
      <a className="skip" href="#content">본문으로 이동</a>
      <header className="top">
        <div className="shell reader-top">
          <a className="brand" href="/" aria-label="dok3search 홈">
            <img className="logo" src="/favicon.svg" alt="" />
            dok3search
          </a>
          <a className="reader-back admin-back" href="/search">
            <ArrowLeft size={14} aria-hidden="true" />
            검색 화면으로 돌아가기
          </a>
        </div>
      </header>

      <main id="content" className="shell admin-shell">
        <div className="admin-header">
          <div>
            <h1>운영 관리</h1>
            <p className="admin-subtitle">로컬 문서 소스와 백그라운드 색인 세대를 관리합니다.</p>
          </div>
          <nav className="admin-tabs" aria-label="관리 메뉴">
            <a
              href="/admin/sources"
              aria-current={tab === 'sources' ? 'page' : undefined}
              onClick={(event) => {
                event.preventDefault()
                switchTab('sources')
              }}
            >
              소스 관리
            </a>
            <a
              href="/admin/jobs"
              aria-current={tab === 'jobs' ? 'page' : undefined}
              onClick={(event) => {
                event.preventDefault()
                switchTab('jobs')
              }}
            >
              색인 작업
            </a>
            <a
              href="/admin/analyzers"
              aria-current={tab === 'analyzers' ? 'page' : undefined}
              onClick={(event) => {
                event.preventDefault()
                switchTab('analyzers')
              }}
            >
              분석기 실험
            </a>
          </nav>
        </div>

        {loading && !accessBanner && !jobsData && (
          <p className="empty" role="status">관리 정보를 불러오는 중입니다.</p>
        )}

        {accessBanner && tab !== 'analyzers' && (
          <section className="admin-alert" role="alert">
            <WarningCircle size={18} aria-hidden="true" />
            <div>
              <strong>접근 또는 실행 모드 안내</strong>
              <p>{accessBanner.message}</p>
            </div>
          </section>
        )}

        {actionNotice && (
          <p className="cls-notice" role="status">{actionNotice}</p>
        )}

        {actionError && (
          <p className="cls-error" role="alert">{actionError}</p>
        )}

        {!accessBanner && jobsData && tab === 'sources' && (
          <section className="admin-section" aria-labelledby="sources-heading">
            <div className="admin-section-top">
              <div>
                <h2 id="sources-heading">등록된 문서 소스 ({sources.length})</h2>
                <p className="admin-meta-line">
                  소스 삭제 시 색인 저장소에서만 제거되며 사용자 디렉터리의 원본 파일은 삭제되지 않습니다.
                </p>
              </div>
              <div className="admin-toolbar">
                <button
                  type="button"
                  className="admin-btn"
                  disabled={busy}
                  onClick={() => void loadAll()}
                >
                  <ArrowsClockwise size={14} aria-hidden="true" />
                  새로고침
                </button>
                <button
                  type="button"
                  className="admin-btn admin-btn-primary"
                  disabled={busy}
                  onClick={() => {
                    setActionError(null)
                    setShowAddForm((prev) => !prev)
                  }}
                >
                  <FolderPlus size={14} aria-hidden="true" />
                  {showAddForm ? '등록 폼 닫기' : '새 소스 등록'}
                </button>
              </div>
            </div>

            {showAddForm && (
              <section className="admin-card" aria-label="새 소스 등록">
                <form className="admin-form" onSubmit={(event) => void handleCreateSource(event)}>
                  <div className="cls-field">
                    <label htmlFor="src-kind">소스 유형</label>
                    <select
                      id="src-kind"
                      value={newKind}
                      disabled={busy}
                      onChange={(event) => {
                        const kind = event.target.value as SourceKind
                        setNewKind(kind)
                        setNewTag(kind === 'sqlite' ? 'sqlite' : 'docs')
                      }}
                    >
                      <option value="local_dir">로컬 문서 폴더</option>
                      <option value="sqlite">SQLite 데이터베이스</option>
                    </select>
                  </div>
                  <div className="admin-form-grid">
                    <div className="cls-field">
                      <label htmlFor="src-id">소스 ID (소문자·숫자·하이픈·밑줄)</label>
                      <input
                        id="src-id"
                        type="text"
                        required
                        pattern="[a-z0-9][a-z0-9_-]{0,63}"
                        placeholder="예: team-runbooks"
                        value={newSourceId}
                        disabled={busy}
                        onChange={(event) => setNewSourceId(event.target.value)}
                      />
                    </div>
                    <div className="cls-field">
                      <label htmlFor="src-label">표시 이름</label>
                      <input
                        id="src-label"
                        type="text"
                        required
                        placeholder="예: 운영 런북 저장소"
                        value={newLabel}
                        disabled={busy}
                        onChange={(event) => setNewLabel(event.target.value)}
                      />
                    </div>
                  </div>
                  <div className="cls-field">
                    <label htmlFor="src-path">
                      {newKind === 'sqlite'
                        ? 'SQLite DB 파일 절대 경로 (읽기 전용, 심볼릭 링크 불가)'
                        : '로컬 디렉터리 절대 경로 (심볼릭 링크 불가)'}
                    </label>
                    <input
                      id="src-path"
                      type="text"
                      required
                      placeholder={newKind === 'sqlite' ? '/path/to/notes.db' : '/path/to/markdown-docs'}
                      value={newPath}
                      disabled={busy}
                      onChange={(event) => setNewPath(event.target.value)}
                    />
                  </div>
                  {newKind === 'sqlite' && (
                    <>
                      <div className="cls-field">
                        <label htmlFor="sq-select">SELECT 쿼리 (읽기 전용)</label>
                        <input
                          id="sq-select"
                          type="text"
                          required
                          value={sqSelect}
                          disabled={busy}
                          onChange={(event) => setSqSelect(event.target.value)}
                        />
                      </div>
                      <div className="admin-form-grid">
                        <div className="cls-field">
                          <label htmlFor="sq-id">ID 컬럼</label>
                          <input id="sq-id" type="text" required value={sqId} disabled={busy} onChange={(event) => setSqId(event.target.value)} />
                        </div>
                        <div className="cls-field">
                          <label htmlFor="sq-title">제목 컬럼</label>
                          <input id="sq-title" type="text" required value={sqTitle} disabled={busy} onChange={(event) => setSqTitle(event.target.value)} />
                        </div>
                        <div className="cls-field">
                          <label htmlFor="sq-body">본문 컬럼</label>
                          <input id="sq-body" type="text" required value={sqBody} disabled={busy} onChange={(event) => setSqBody(event.target.value)} />
                        </div>
                        <div className="cls-field">
                          <label htmlFor="sq-summary">요약 컬럼 (선택)</label>
                          <input id="sq-summary" type="text" value={sqSummary} disabled={busy} onChange={(event) => setSqSummary(event.target.value)} />
                        </div>
                        <div className="cls-field">
                          <label htmlFor="sq-cursor-mode">커서 방식</label>
                          <select
                            id="sq-cursor-mode"
                            value={sqCursorMode}
                            disabled={busy}
                            onChange={(event) => setSqCursorMode(event.target.value as SqliteCursorMode)}
                          >
                            <option value="integer_pk">정수 기본키 (신규 행만)</option>
                            <option value="updated_at">수정 시각 (신규·변경 행)</option>
                          </select>
                        </div>
                        <div className="cls-field">
                          <label htmlFor="sq-cursor-column">커서 컬럼</label>
                          <input id="sq-cursor-column" type="text" required value={sqCursorColumn} disabled={busy} onChange={(event) => setSqCursorColumn(event.target.value)} />
                        </div>
                      </div>
                    </>
                  )}
                  <div className="admin-form-grid">
                    <div className="cls-field">
                      <label htmlFor="src-category">기본 카테고리</label>
                      <select
                        id="src-category"
                        value={newCategory}
                        disabled={busy}
                        onChange={(event) => setNewCategory(event.target.value)}
                      >
                        {categories.map((item) => (
                          <option key={item.id} value={item.id}>{item.label} ({item.id})</option>
                        ))}
                      </select>
                    </div>
                    <div className="cls-field">
                      <label htmlFor="src-tag">기본 태그</label>
                      <input
                        id="src-tag"
                        type="text"
                        value={newTag}
                        disabled={busy}
                        onChange={(event) => setNewTag(event.target.value)}
                      />
                    </div>
                  </div>
                  <div className="cls-actions">
                    <button type="submit" className="admin-btn admin-btn-primary" disabled={busy}>
                      <FolderPlus size={14} aria-hidden="true" />
                      {busy ? '등록 및 색인 중...' : '등록 및 즉시 색인'}
                    </button>
                    <button
                      type="button"
                      className="admin-btn"
                      disabled={busy}
                      onClick={() => setShowAddForm(false)}
                    >
                      <X size={14} aria-hidden="true" />
                      취소
                    </button>
                  </div>
                </form>
              </section>
            )}

            <div className="admin-list">
              {sources.map((src) => (
                <article className="admin-card" key={src.sourceId}>
                  <div className="admin-card-head">
                    <div>
                      <div className="admin-title-row">
                        <h3>{src.label}</h3>
                        <code className="admin-code">{src.sourceId}</code>
                        {src.kind === 'sqlite' && (
                          <span className="admin-badge">
                            <Database size={12} aria-hidden="true" />
                            SQLite
                          </span>
                        )}
                        {(src.kind === 'postgres' || src.kind === 'mysql') && (
                          <span className="admin-badge">
                            <Database size={12} aria-hidden="true" />
                            {src.kind === 'postgres' ? 'PostgreSQL' : 'MySQL'}
                          </span>
                        )}
                        {src.managedByEnv && (
                          <span className="admin-badge">
                            <LockSimple size={12} aria-hidden="true" />
                            환경변수 관리
                          </span>
                        )}
                        <span className="admin-badge admin-badge-muted">
                          {categoryLabels.get(src.defaultCategory as typeof categories[number]['id']) ?? src.defaultCategory}
                        </span>
                      </div>
                      <p className="admin-path">{src.path}</p>
                    </div>
                    <div className="admin-card-actions">
                      <button
                        type="button"
                        className="admin-btn"
                        disabled={busy}
                        onClick={() => void handleTriggerJob('incremental', src.sourceId)}
                      >
                        <ArrowsClockwise size={14} aria-hidden="true" />
                        재스캔
                      </button>
                      {!src.managedByEnv && (
                        <button
                          type="button"
                          className="admin-btn"
                          disabled={busy}
                          onClick={() =>
                            setConfirmDeleteId((prev) => (prev === src.sourceId ? null : src.sourceId))
                          }
                        >
                          <Trash size={14} aria-hidden="true" />
                          삭제
                        </button>
                      )}
                    </div>
                  </div>
                  <div className="admin-stats-row">
                    <span>색인 문서: <strong>{src.documentCount}</strong>건</span>
                    <span>기본 태그: <code>{src.defaultTag}</code></span>
                    <span>최근 동기화: {formatTimestamp(src.lastSyncedAtMs)}</span>
                  </div>
                  {src.lastError && (
                    <p className="cls-error" role="alert">최근 스캔 오류: {src.lastError}</p>
                  )}
                  {confirmDeleteId === src.sourceId && (
                    <div className="admin-confirm">
                      <p>
                        <strong>{src.label}</strong> ({src.sourceId}) 등록을 삭제할까요? 색인 데이터에서만 제거되며 디스크 파일은 지워지지 않습니다.
                      </p>
                      <div className="cls-actions">
                        <button
                          type="button"
                          className="admin-btn admin-btn-primary"
                          disabled={busy}
                          onClick={() => void handleDeleteSource(src.sourceId)}
                        >
                          삭제 확정
                        </button>
                        <button
                          type="button"
                          className="admin-btn"
                          disabled={busy}
                          onClick={() => setConfirmDeleteId(null)}
                        >
                          취소
                        </button>
                      </div>
                    </div>
                  )}
                </article>
              ))}
            </div>
          </section>
        )}

        {!accessBanner && jobsData && tab === 'jobs' && (
          <section className="admin-section" aria-labelledby="jobs-heading">
            <div className="admin-section-top">
              <div>
                <h2 id="jobs-heading">색인 작업 이력 ({jobsData.jobs.length})</h2>
                <p className="admin-meta-line">
                  <Database size={14} aria-hidden="true" />
                  활성 세대: <code>{jobsData.activeGeneration ?? '없음'}</code> · Epoch {jobsData.activeEpoch} · 주기적 갱신:{' '}
                  {jobsData.refreshIntervalSecs ? `${jobsData.refreshIntervalSecs}초` : '비활성'}
                </p>
              </div>
              <div className="admin-toolbar">
                <button
                  type="button"
                  className="admin-btn admin-btn-primary"
                  disabled={busy}
                  onClick={() => void handleTriggerJob('incremental')}
                >
                  <Play size={14} aria-hidden="true" />
                  증분 새로고침
                </button>
                <button
                  type="button"
                  className="admin-btn"
                  disabled={busy}
                  onClick={() => void handleTriggerJob('full')}
                >
                  <ArrowsClockwise size={14} aria-hidden="true" />
                  전체 재색인
                </button>
                <button
                  type="button"
                  className="admin-btn"
                  disabled={busy}
                  onClick={() => void loadAll()}
                >
                  목록 갱신
                </button>
              </div>
            </div>

            {jobsData.jobs.length === 0 ? (
              <p className="empty">기록된 색인 작업이 없습니다.</p>
            ) : (
              <div className="admin-table-wrap">
                <table className="admin-table">
                  <thead>
                    <tr>
                      <th>작업 ID</th>
                      <th>상태</th>
                      <th>트리거 · 모드</th>
                      <th>대상 소스</th>
                      <th>문서 / Epoch</th>
                      <th>실행 시각</th>
                    </tr>
                  </thead>
                  <tbody>
                    {jobsData.jobs.map((job) => (
                      <tr key={job.jobId}>
                        <td><code>{job.jobId}</code></td>
                        <td>
                          <span className={job.state === 'failed' ? 'admin-badge admin-badge-strong' : 'admin-badge'}>
                            {job.state === 'succeeded' && <CheckCircle size={12} aria-hidden="true" />}
                            {job.state === 'failed' && <WarningCircle size={12} aria-hidden="true" />}
                            {job.state}
                          </span>
                        </td>
                        <td>{job.trigger} · {job.mode}</td>
                        <td><code>{job.sourceId ?? '전체'}</code></td>
                        <td>{job.indexedDocuments}건 · epoch {job.activeEpoch}{job.rebuilt ? ' (재구축)' : ''}</td>
                        <td>
                          {formatTimestamp(job.startedAtMs)}
                          {(job.error || job.skippedFiles.length > 0) && (
                            <details className="admin-details">
                              <summary>
                                {job.error ? '오류 상세' : `스킵/경고 (${job.skippedFiles.length})`}
                              </summary>
                              {job.error && <p className="cls-error">{job.error}</p>}
                              {job.skippedFiles.length > 0 && (
                                <ul>
                                  {job.skippedFiles.map((item, idx) => (
                                    <li key={`${job.jobId}-skip-${idx}`}>{item}</li>
                                  ))}
                                </ul>
                              )}
                            </details>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </section>
        )}
        {tab === 'analyzers' && (
          <section className="admin-section" aria-labelledby="analyzers-heading">
            <div className="admin-section-top">
              <div>
                <h2 id="analyzers-heading">검색어 분석기 실험</h2>
                <p className="admin-meta-line">
                  현재 활성화된 형태소·식별자·n-gram 분석 결과를 확인합니다.
                </p>
              </div>
            </div>
            <section className="admin-card">
              <form className="admin-form" onSubmit={(event) => void handleAnalyze(event)}>
                <div className="cls-field">
                  <label htmlFor="analyze-input">분석할 텍스트 또는 질의어</label>
                  <textarea
                    id="analyze-input"
                    rows={3}
                    required
                    value={analyzeInput}
                    disabled={busy}
                    onChange={(event) => setAnalyzeInput(event.target.value)}
                  />
                </div>
                <div className="cls-actions">
                  <label>
                    <input
                      type="checkbox"
                      checked={analyzeSplit}
                      onChange={(event) => setAnalyzeSplit(event.target.checked)}
                    />{' '}
                    식별자·복합어 분해
                  </label>
                  <label>
                    <input
                      type="checkbox"
                      checked={analyzeNgrams}
                      onChange={(event) => setAnalyzeNgrams(event.target.checked)}
                    />{' '}
                    문자 n-gram 생성
                  </label>
                  <button type="submit" className="admin-btn admin-btn-primary" disabled={busy}>
                    <Play size={14} aria-hidden="true" />
                    분석 실행
                  </button>
                </div>
              </form>
              {analyzeResult && (
                <div className="admin-stats-row">
                  <span>프로파일: <code>{analyzeResult.profile}</code> (v{analyzeResult.analyzerVersion})</span>
                  <span>정규화 질의: <code>{analyzeResult.normalizedQueryTerms.join(', ') || '(없음)'}</code></span>
                  <span>색인 토큰({analyzeResult.terms.length}): <code>{analyzeResult.terms.join(', ') || '(없음)'}</code></span>
                  <span>n-gram({analyzeResult.charNgrams.length}): <code>{analyzeResult.charNgrams.slice(0, 24).join(', ') || '(없음)'}</code></span>
                </div>
              )}
            </section>
          </section>
        )}
      </main>
    </>
  )
}
