// INTRO:

// The bit is the foundational unit of data in computing.
// From the bites, we build bytes, and from bytes, we construct complex data structures.
// 1 bytes = 8 bits

// Binary is a base-2 numeral system that uses two symbols, typically 0 and 1.
// Each digit in a binary number is referred to as a bit.

// 1. Converting binary to decimal

// Binary place values from right to left:
// 128, 64, 32, 16, 8, 4, 2, 1
// or 2^7, 2^6, 2^5, 2^4, 2^3, 2^2, 2^1, 2^0

// For example, the binary 1011 in decimal is
// 1 * 2^3 + 0 * 2^2 + 1 * 2^1 + 1 * 2^0 = 8 + 0 + 2 + 1 = 11

// Full byte of 00110001
// 1 * 2^5 + 1 * 2^4 + 1 * 2^0 = 32 + 16 + 1 = 49

function fromBinaryToDecimal(binary) {
  if (typeof binary !== "string") throw Error("Binary arg should be string");
  const reversedBinary = binary.split("").reverse().join("");

  let decimal = 0;

  for (const [index, bit] of [...reversedBinary].entries()) {
    const bitNum = parseInt(bit);
    if (Number.isNaN(bitNum)) {
      throw Error(`Bit ${bit} is not a valid number!`);
    }

    const sum = bitNum * 2 ** index;
    decimal += sum;
  }

  return decimal;
}

console.log("From binary to decimal", fromBinaryToDecimal("00110001"));

// When converting decimal to binary we use reminders and MSB
// For example, 16
// 16 % 2 = 0
// 8 % 2 = 0
// 4 % 2 = 0
// 2 % 2 = 0
// 1 % 2 = 1
// So the binary representation is 10000

function fromDecimalToBinary(decimal, padStart = false) {
  if (typeof decimal !== "number") throw Error("Decimal is not a number");

  let quotient = decimal;
  let binary = "";

  while (quotient > 0) {
    let reminder = quotient % 2;
    quotient = Math.floor(quotient / 2);
    binary += reminder;
  }
  if (!padStart) return binary.split("").reverse().join("");
  return binary.split("").reverse().join("").padStart(8, "0");
}

console.log("From decimal to binary", fromDecimalToBinary(49, true));

// 2. Converting binary to hexadecimal and vice versa

// Hex is base 16, meaning, each place value is a power of 16
// It goes 0-9 and then A-F
const hexDigits = [
  "0",
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
];

function fromHexToBinary(hex, byteSplit = true) {
  if (typeof hex !== "string") throw Error("Provided HEX is not string");

  const hexSplit = hex.split("");
  let finalBinary = "";

  for (const hexEl of hexSplit) {
    const targetIdx = hexDigits.indexOf(hexEl);
    if (targetIdx === -1) throw Error(`Index of ${hexEl} is not found`);

    const binary = fromDecimalToBinary(targetIdx);
    finalBinary += binary;
  }
  if (byteSplit) return finalBinary.match(/.{1,8}/g);
  return finalBinary;
}

console.log("From HEX to binary", fromHexToBinary("AC"));

// When we convert from binary to hex, we take 4 bits binary (range 0000 to 1111) that
// represents 0 to 15, exactly what HEX covers

function fromBinaryToHex(binary) {
  if (typeof binary !== "string") throw Error("Binary input is not a string");
  // split binary into chunks of 4
  const chunks = binary.match(/.{1,4}/g);

  let finalHex = "";

  for (const chunk of chunks) {
    const decimal = fromBinaryToDecimal(chunk);
    const hexByIndex = hexDigits.at(decimal);
    if (!hexByIndex) throw Error(`Hex by index is not found for ${decimal}`);

    finalHex += hexByIndex;
  }

  return finalHex;
}

console.log("From binary to HEX", fromBinaryToHex("10101100"));
