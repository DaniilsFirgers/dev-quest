function fromDecimalToBinary(decimal) {
  if (decimal === 0) return "0";
  let binary = "";
  while (decimal > 0) {
    binary += (decimal % 2).toString();
    decimal = Math.floor(decimal / 2);
  }
  return binary.split("").reverse().join("");
}

const decimalNumbers = [0, 1, 2, 3, 4, 5, 10, 15, 16, 31];

decimalNumbers.forEach((num) => {
  console.log(`Decimal: ${num}, Binary: ${fromDecimalToBinary(num)}`);
});

function fromBinaryToDecimal(binary) {
  const binaryString = binary.toString();

  let decimal = 0;
  for (let i = 0; i < binaryString.length; i++) {
    const bit = binaryString[binaryString.length - 1 - i];
    if (bit === "1") {
      decimal += Math.pow(2, i);
    }
  }
  return decimal;
}

const binaries = [101, 110, 111, 1001, 1010, 1110, 1111, 10000, 10001, 10010];

binaries.forEach((binary) => {
  console.log(`Binary: ${binary}, Decimal: ${fromBinaryToDecimal(binary)}`);
});
