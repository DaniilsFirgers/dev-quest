// Conversion from decimal to binary works in the following way:
// 1. Divide the number by 2.
// 2. Write down the remainder (it will always be 1 or 0).
// 3. Repeat the process with the quotient until the quotient is 0.
// 4. Flip the remainders to get the binary representation. Like
//    13 / 2 = 6 remainder 1
//    6 / 2 = 3 remainder 0
//    3 / 2 = 1 remainder 1
//    1 / 2 = 0 remainder 1
//    Flipping the remainders we get 1101 which is the binary representation of 13.
function fromDecimalToBinary(decimal) {
  if (decimal === 0) return "0".repeat(8);
  let binary = "";
  while (decimal > 0) {
    binary += (decimal % 2).toString();
    decimal = Math.floor(decimal / 2);
  }
  // Pad with leading zeros to ensure 8 bits
  return binary.split("").reverse().join("").padStart(8, "0");
}

const decimalNumbers = [0, 1, 2, 3, 4, 5, 10, 15, 16, 31];
// decimalNumbers.forEach((num) => {
//   console.log(`Decimal: ${num}, Binary: ${fromDecimalToBinary(num)}`);
// });

// Conversion from binary to decimal works in the following way:
// 1. Write down the binary number.
// 2. Starting from the right multiply each bit by 2 raised to the power of its position (starting from 0).
// 3. Sum all the results to get the decimal representation. Like
//    1101 = (1 * 2^3) + (1 * 2^2) + (0 * 2^1) + (1 * 2^0) = 8 + 4 + 0 + 1 = 13
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
// binaries.forEach((binary) => {
//   console.log(`Binary: ${binary}, Decimal: ${fromBinaryToDecimal(binary)}`);
// });

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

// console.log(exclusiveOr(14, 10)); // Output: "0110"

// Simple encryption/decryption using XOR cipher. which works because A ^ B ^ B = A
//
function encryptDecrypt(message, key) {
  let result = "";
  for (let i = 0; i < message.length; i++) {
    // convert each character to its ASCII value like 'A' = 65
    const msgAsciiChar = String(message).charCodeAt(i);
    // if the key is shorter than the message, we loop over the key
    const keyAsciiChar = String(key).charCodeAt(i % key.length);
    const encryptedChar = exclusiveOr(msgAsciiChar, keyAsciiChar);

    // convert the encrypted ASCII value back to a character
    result += String.fromCharCode(encryptedChar);
  }
  return result;
}

const cipher = encryptDecrypt("hello", "123");
console.log("cipher", cipher);

const original = encryptDecrypt(cipher, "123");
console.log("original", original);
