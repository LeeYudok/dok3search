# Polyglot Enterprise & Web Code Samples

폐쇄망 환경에서 `dok3search`가 **Java · JSP · JavaScript(CJS) · TypeScript · React(TSX) · Next.js App Router** 소스코드를 정확히 파싱·분류·경로 식별·검색하는지 검증하기 위한 MIT 라이선스 대표 샘플 팩입니다.

## 포함 스택

- `java/OrderController.java`, `java/OrderRepository.java`: Spring `@RestController`, `record`, 인터페이스 `;` 메서드, JDBC `ORA-00904` 폴백 복구
- `jsp/orderList.jsp`: `<%@ page/taglib %>`, `<%! %>` 선언부, `<form action="/api/orders/search">`, `<c:forEach>`, `<script>`
- `js/legacyOrderClient.cjs`: CommonJS `exports.*`, 지수 백오프 재시도, `X-Idempotency-Key`
- `ts/orderSettlement.ts`: TypeScript 유니온/인터페이스, JSDoc, `calculateSettlement`
- `react/OrderStatusBadge.tsx`: `useOrderPolling` 커스텀 훅, `memo(forwardRef(...))`, `aria-live="polite"`
- `nextjs/app/{orders,users}/page.tsx`, `nextjs/app/api/orders/route.ts`: App Router 동일 파일명(`page.tsx`) 경로 식별 및 `GET`/`POST` Route Handler
