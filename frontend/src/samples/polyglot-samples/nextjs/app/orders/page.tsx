import { OrderStatusBadge } from '../../react/OrderStatusBadge';

export async function generateMetadata() {
  return {
    title: '주문 정산 대시보드',
  };
}

/**
 * Next.js App Router 주문 목록 페이지 (/orders).
 */
export default async function OrdersPage() {
  return (
    <main>
      <h1>주문 정산 현황</h1>
      <OrderStatusBadge orderId="ORD-1001" initialStatus="SETTLED" />
    </main>
  );
}
