package com.doksam.orders.web;

import com.doksam.orders.repo.OrderRepository;
import com.doksam.orders.repo.OrderRepository.OrderRow;
import java.sql.SQLException;
import java.util.List;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

/**
 * 주문 접수 및 정산 조회 REST 컨트롤러.
 * 폐쇄망 기업 백엔드(Spring Boot + JDBC) 표준 패턴 샘플입니다.
 */
@RestController
@RequestMapping("/api/orders")
public class OrderController {

    private final OrderRepository orderRepository;

    public OrderController(OrderRepository orderRepository) {
        this.orderRepository = orderRepository;
    }

    /**
     * 신규 주문 생성 요청 페이로드.
     */
    public record OrderCreateRequest(
            String customerId,
            long grossAmountKrw,
            String idempotencyKey
    ) {}

    /**
     * 신규 주문을 생성하고 초기 정산 대기 상태로 저장합니다.
     */
    @PostMapping
    public ResponseEntity<OrderRow> createOrder(@RequestBody OrderCreateRequest request)
            throws SQLException {
        if (request.idempotencyKey() == null || request.idempotencyKey().isBlank()) {
            return ResponseEntity.status(HttpStatus.CONFLICT).build();
        }
        OrderRow created = orderRepository.insertPendingOrder(
                request.customerId(),
                request.grossAmountKrw(),
                request.idempotencyKey()
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(created);
    }

    /**
     * 상태별 주문 목록을 조회합니다.
     */
    @GetMapping("/search")
    public ResponseEntity<List<OrderRow>> searchOrders(
            @RequestParam(defaultValue = "SETTLED") String status
    ) throws SQLException {
        return ResponseEntity.ok(orderRepository.findOrdersByStatus(status));
    }

    /**
     * 단건 주문 정산 내역을 반환합니다.
     */
    @GetMapping("/{orderId}/settlement")
    public ResponseEntity<OrderRow> getOrderSettlement(@PathVariable String orderId)
            throws SQLException {
        return orderRepository.findByOrderId(orderId)
                .map(ResponseEntity::ok)
                .orElseGet(() -> ResponseEntity.notFound().build());
    }
}
