function fromDecimalToBinary(decimal) {
  if (decimal === 0) return "0".repeat(8);
  let binary = "";
  while (decimal > 0) {
    binary += (decimal % 2).toString();
    decimal = Math.floor(decimal / 2);
  }
  return binary.split("").reverse().join("").padStart(8, "0");
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

function exclusiveOr(a, b) {
  const binaryA = fromDecimalToBinary(a);
  const binaryB = fromDecimalToBinary(b);
  let result = "";
  for (let i = 0; i < 8; i++) {
    if (binaryA[i] === binaryB[i]) {
      result += "0";
    } else {
      result += "1";
    }
  }
  return fromBinaryToDecimal(result);
}

function xorProperty(a, b) {
  return (a ^ b) === exclusiveOr(a, b);
}

// console.log(exclusiveOr(14, 10)); // Output: "0110"
// console.log(xorProperty(14, 10)); // Output: true

/// Encrypt message using XOR
function encryptDecrypt(message, key) {
  let result = "";
  for (let i = 0; i < message.length; i++) {
    const msgAsciiChar = String(message).charCodeAt(i);
    const keyAsciiChar = String(key).charCodeAt(i % key.length);
    const encryptedChar = exclusiveOr(msgAsciiChar, keyAsciiChar);

    result += String.fromCharCode(encryptedChar);
  }
  return result;
}

const cipher = encryptDecrypt("hello", "123");
console.log("cipher", cipher);

const original = encryptDecrypt(cipher, "123");
console.log("original", original);
