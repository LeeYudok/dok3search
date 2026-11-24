import { NextRequest, NextResponse } from 'next/server';
import { calculateSettlement } from '../../../ts/orderSettlement';

/**
 * 주문 목록 조회 Route Handler (GET /api/orders).
 */
export async function GET(request: NextRequest) {
  const status = request.nextUrl.searchParams.get('status') ?? 'SETTLED';
  return NextResponse.json({
    status,
    orders: [{ orderId: 'ORD-1001', grossAmountKrw: 120_000 }],
  });
}

/**
 * 신규 주문 생성 Route Handler (POST /api/orders).
 * x-idempotency-key 헤더가 없으면 409 Conflict를 반환합니다.
 */
export async function POST(request: NextRequest) {
  const idempotencyKey = request.headers.get('x-idempotency-key')?.trim();
  if (!idempotencyKey) {
    return NextResponse.json(
      { error: 'missing x-idempotency-key header' },
      { status: 409 },
    );
  }
  const body = (await request.json()) as { grossAmountKrw: number; feeBps?: number };
  const settlement = calculateSettlement(body.grossAmountKrw, body.feeBps ?? 300);
  return NextResponse.json({ idempotencyKey, settlement }, { status: 201 });
}
