'use strict';

/**
 * 지수 백오프로 일시적 502/503 오류를 재시도합니다.
 * @param {() => Promise<Response>} fn
 * @param {number} maxAttempts
 */
exports.retryWithBackoff = async (fn, maxAttempts = 3) => {
  let attempt = 0;
  while (attempt < maxAttempts) {
    attempt += 1;
    const response = await fn();
    if (response.status !== 502 && response.status !== 503) {
      return response;
    }
    if (attempt >= maxAttempts) {
      return response;
    }
    await new Promise((resolve) => setTimeout(resolve, 100 * 2 ** (attempt - 1)));
  }
  throw new Error('unreachable');
};

/**
 * 멱등성 키(X-Idempotency-Key)를 포함해 주문 요약 API를 호출합니다.
 */
exports.fetchOrderSummary = async function fetchOrderSummary(baseUrl, orderId, idempotencyKey) {
  const url = `${baseUrl.replace(/\/$/, '')}/api/orders/${encodeURIComponent(orderId)}/settlement`;
  const response = await exports.retryWithBackoff(() =>
    fetch(url, {
      method: 'GET',
      headers: {
        Accept: 'application/json',
        'X-Idempotency-Key': idempotencyKey,
      },
    }),
  );
  if (!response.ok) {
    throw new Error(`order summary failed: ${response.status}`);
  }
  return response.json();
};
