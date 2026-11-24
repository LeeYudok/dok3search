<%@ page contentType="text/html; charset=UTF-8" pageEncoding="UTF-8" %>
<%@ taglib prefix="c" uri="jakarta.tags.core" %>
<%-- 레거시 JSP 주문 조회 화면: Spring OrderController(/api/orders/search)와 연동 --%>
<%!
  public String formatKrw(long amount) {
    return String.format("%,d원", amount);
  }
%>
<!DOCTYPE html>
<html lang="ko">
<head>
  <meta charset="UTF-8" />
  <title>주문 정산 목록</title>
</head>
<body>
  <form id="orderSearchForm" action="/api/orders/search" method="get">
    <label for="status">주문 상태</label>
    <select id="status" name="status">
      <option value="SETTLED">정산완료</option>
      <option value="PENDING">대기</option>
    </select>
    <button type="submit">조회</button>
  </form>

  <table>
    <tbody>
      <c:forEach items="${orderRows}" var="orderRow">
        <tr data-order-id="${orderRow.orderId}">
          <td>${orderRow.orderId}</td>
          <td>${orderRow.customerId}</td>
          <td>${orderRow.status}</td>
          <td><%= formatKrw(0L) %></td>
        </tr>
      </c:forEach>
    </tbody>
  </table>

  <script>
    document.getElementById('orderSearchForm')?.addEventListener('submit', function (event) {
      var statusEl = document.getElementById('status');
      if (!statusEl || !statusEl.value) {
        event.preventDefault();
      }
    });
  </script>
</body>
</html>
