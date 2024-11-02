import { RequestMessage, ResponseMessage, Topics } from "./types";

const TOPICS: { [key in Topics]: (msg: RequestMessage) => ResponseMessage } = {
  "rpc.add": add,
  "rpc.subtract": subtract,
  "rpc.multiply": multiply,
  "rpc.divide": divide,
};

function add(msg: RequestMessage): ResponseMessage {
  return { result: msg.num1 + msg.num2 };
}

function subtract(msg: RequestMessage): ResponseMessage {
  return { result: msg.num1 - msg.num2 };
}

function multiply(msg: RequestMessage): ResponseMessage {
  return { result: msg.num1 * msg.num2 };
}

function divide(msg: RequestMessage): ResponseMessage {
  if (msg.num2 == 0) return { result: null, error: "Cannot divide by zero" };
  return { result: msg.num1 / msg.num2 };
}

export { TOPICS };
