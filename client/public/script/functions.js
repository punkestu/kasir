function trimLeftZero(value) {
  return value.replace(/^0+/, "");
}
function zeroOnEmpty(value) {
  return value.length > 0 ? value : "0";
}
function zerOnNegative(value) {
  return parseInt(value) < 0 ? "0" : value;
}