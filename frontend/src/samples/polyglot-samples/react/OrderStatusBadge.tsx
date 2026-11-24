import { forwardRef, memo, useEffect, useState } from 'react';
import type { OrderStatus } from '../ts/orderSettlement';

export interface OrderStatusBadgeProps {
  orderId: string;
  initialStatus: OrderStatus;
  pollIntervalMs?: number;
}

/**
 * 주문 상태를 주기적으로 폴링하는 커스텀 React 훅입니다.
 */
export function useOrderPolling(orderId: string, intervalMs = 5000): OrderStatus {
  const [status, setStatus] = useState<OrderStatus>('PENDING');

  useEffect(() => {
    let active = true;
    const timer = window.setInterval(async () => {
      try {
        const res = await fetch(`/api/orders/${encodeURIComponent(orderId)}/settlement`);
        if (res.ok && active) {
          const data = (await res.json()) as { status?: OrderStatus };
          if (data.status) {
            setStatus(data.status);
          }
        }
      } catch {
        // 폴링 실패 시 다음 주기에 재시도합니다.
      }
    }, intervalMs);

    return () => {
      active = false;
      window.clearInterval(timer);
    };
  }, [orderId, intervalMs]);

  return status;
}

/**
 * 스크린 리더 접근성(role="status", aria-live="polite")을 갖춘 주문 상태 배지 컴포넌트.
 */
export const OrderStatusBadge = memo(
  forwardRef<HTMLSpanElement, OrderStatusBadgeProps>(function OrderStatusBadge(
    { orderId, initialStatus, pollIntervalMs = 5000 },
    ref,
  ) {
    const liveStatus = useOrderPolling(orderId, pollIntervalMs);
    const displayStatus = liveStatus || initialStatus;
    return (
      <span ref={ref} role="status" aria-live="polite" data-order-id={orderId}>
        {displayStatus}
      </span>
    );
  }),
);
