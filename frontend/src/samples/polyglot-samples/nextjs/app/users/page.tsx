export async function generateMetadata() {
  return {
    title: '운영자 계정 목록',
  };
}

/**
 * Next.js App Router 사용자 관리 페이지 (/users).
 * orders/page.tsx 와 경로 기반 제목 식별을 검증하기 위한 대조 라우트입니다.
 */
export default async function UsersPage() {
  return (
    <main>
      <h1>운영자 계정 목록</h1>
    </main>
  );
}
