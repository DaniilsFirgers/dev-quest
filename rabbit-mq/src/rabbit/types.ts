enum ContentType {
  JSON = "application/json",
  TEXT = "text/plain",
  XML = "application/xml",
}

enum Action {
  MULTIPLY = "multiply",
  ADD = "add",
  SUBTRACT = "subtract",
  DIVIDE = "divide",
}

type RequestMessage = {
  num1: number;
  num2: number;
};

type ResponseMessage = {
  result: number | null;
  error?: string;
};

type Topics = "rpc.add" | "rpc.subtract" | "rpc.multiply" | "rpc.divide";

export {
  ContentType,
  Action,
  type RequestMessage,
  type ResponseMessage,
  type Topics,
};
