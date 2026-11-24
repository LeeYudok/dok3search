export type OrderStatus = 'PENDING' | 'SETTLED' | 'FAILED';

export interface SettlementBreakdown {
  grossAmountKrw: number;
  feeKrw: number;
  vatKrw: number;
  netAmountKrw: number;
}

/**
 * 총 주문 금액(KRW)과 수수료율(bps)을 기준으로 정산 내역을 계산합니다.
 * 수수료는 원 단위 미만 버림(Math.floor), 부가세(VAT)는 수수료의 10%를 적용합니다.
 */
export function calculateSettlement(
  grossAmountKrw: number,
  feeBps: number,
): SettlementBreakdown {
  if (grossAmountKrw < 0 || feeBps < 0) {
    throw new RangeError('grossAmountKrw and feeBps must be non-negative');
  }
  const feeKrw = Math.floor((grossAmountKrw * feeBps) / 10_000);
  const vatKrw = Math.floor(feeKrw * 0.1);
  const netAmountKrw = grossAmountKrw - feeKrw - vatKrw;
  return {
    grossAmountKrw,
    feeKrw,
    vatKrw,
    netAmountKrw,
  };
}
