function IncQty(id) {
  const qty = document.querySelector(id);
  qty.value = [(parseInt(qty.value) + 1).toString()].flatMap(trimLeftZero)[0];
}
function DecQty(id) {
  const qty = document.querySelector(id);
  qty.value = [(parseInt(qty.value) - 1).toString()]
    .flatMap(trimLeftZero)
    .flatMap(zeroOnEmpty)
    .flatMap(zerOnNegative)[0];
}
