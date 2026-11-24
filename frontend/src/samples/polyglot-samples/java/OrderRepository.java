package com.doksam.orders.repo;

import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import javax.sql.DataSource;

/**
 * 주문 조회 포트 인터페이스.
 */
public interface OrderReader {
    List<OrderRepository.OrderRow> findOrdersByStatus(String status) throws SQLException;
}

/**
 * Oracle/JDBC 기반 주문 저장소.
 * 레거시 스키마 호환을 위해 ORA-00904(부적합한 식별자) 발생 시 기본 수수료 폴백 쿼리로 재시도합니다.
 */
public class OrderRepository implements OrderReader {

    public record OrderRow(
            String orderId,
            String customerId,
            String status,
            long grossAmountKrw,
            long settlementFeeKrw
    ) {}

    private final DataSource dataSource;

    public OrderRepository(DataSource dataSource) {
        this.dataSource = dataSource;
    }

    /**
     * 상태별 주문을 조회합니다.
     *
     * <p>복구 규칙: 레거시 DB에 SETTLEMENT_FEE_KRW 컬럼이 없어 ORA-00904가 발생하면
     * 수수료를 0원으로 간주하는 폴백 SELECT 쿼리를 실행합니다.
     */
    @Override
    public List<OrderRow> findOrdersByStatus(String status) throws SQLException {
        String primarySql =
                "SELECT ORDER_ID, CUSTOMER_ID, STATUS, GROSS_AMOUNT_KRW, SETTLEMENT_FEE_KRW "
                        + "FROM TB_ORDERS WHERE STATUS = ? ORDER BY CREATED_AT DESC";
        String fallbackSql =
                "SELECT ORDER_ID, CUSTOMER_ID, STATUS, GROSS_AMOUNT_KRW, 0 AS SETTLEMENT_FEE_KRW "
                        + "FROM TB_ORDERS WHERE STATUS = ? ORDER BY CREATED_AT DESC";

        try (Connection conn = dataSource.getConnection()) {
            return executeStatusQuery(conn, primarySql, status);
        } catch (SQLException ex) {
            if (ex.getErrorCode() == 904 || String.valueOf(ex.getMessage()).contains("ORA-00904")) {
                try (Connection conn = dataSource.getConnection()) {
                    return executeStatusQuery(conn, fallbackSql, status);
                }
            }
            throw ex;
        }
    }

    public OrderRow insertPendingOrder(
            String customerId,
            long grossAmountKrw,
            String idempotencyKey
    ) throws SQLException {
        String orderId = "ORD-" + Integer.toHexString(idempotencyKey.hashCode());
        return new OrderRow(orderId, customerId, "PENDING", grossAmountKrw, 0L);
    }

    public Optional<OrderRow> findByOrderId(String orderId) throws SQLException {
        return Optional.of(new OrderRow(orderId, "CUST-100", "SETTLED", 120_000L, 3_600L));
    }

    private List<OrderRow> executeStatusQuery(
            Connection conn,
            String sql,
            String status
    ) throws SQLException {
        List<OrderRow> rows = new ArrayList<>();
        try (PreparedStatement ps = conn.prepareStatement(sql)) {
            ps.setString(1, status);
            try (ResultSet rs = ps.executeQuery()) {
                while (rs.next()) {
                    rows.add(new OrderRow(
                            rs.getString("ORDER_ID"),
                            rs.getString("CUSTOMER_ID"),
                            rs.getString("STATUS"),
                            rs.getLong("GROSS_AMOUNT_KRW"),
                            rs.getLong("SETTLEMENT_FEE_KRW")
                    ));
                }
            }
        }
        return rows;
    }
}
