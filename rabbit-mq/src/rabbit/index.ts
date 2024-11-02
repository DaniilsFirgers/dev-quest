import amqp from "amqplib";
import config from "../config.js";
import Client from "./client.js";

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
  action: Action;
  num1: number;
  num2: number;
};

type ResponseMessage = {
  result: number | null;
  error?: string;
};

class Rabbit {
  static isInitialized = false;
  static connection: amqp.Connection;
  static consumeServerChannel: amqp.Channel;
  static publishServerChannel: amqp.Channel;

  private constructor() {}

  static async initialize() {
    try {
      this.connection = await amqp.connect(
        `${config.rabbitMq.protocol}://${config.rabbitMq.host}:${config.rabbitMq.port}`
      );
      // NOTE: it is recommended to create a separate channel for producing and consuming
      // of messages for better performance, avoid blocking the channel
      // Channels are lightweight and can be created and destroyed easily
      // max number of channels per connection is 65535
      this.consumeServerChannel = await this.connection.createChannel();
      this.publishServerChannel = await this.connection.createChannel();

      // assert the exchange
      // exchange is set to durable so that it can survive a server restart
      // it will be direct exchange type
      await this.consumeServerChannel.assertExchange(
        config.rabbitMq.rpc.exchange,
        "direct",
        {
          durable: true,
        }
      );

      // now we have to create an rpc queue
      // queue is set to durable so that it can survive a server restart
      // however messages should be set to persistent to survive a server restart
      const q = await this.consumeServerChannel.assertQueue(
        config.rabbitMq.rpc.queue,
        {
          durable: true,
        }
      );

      // messsages will be consumed sent with a specific routing key
      this.consumeServerChannel.bindQueue(
        q.queue,
        config.rabbitMq.rpc.exchange,
        config.rabbitMq.rpc.routingKey
      );

      this.isInitialized = true;

      // start the server
      this.initServer(q.queue);
    } catch (error) {
      throw new Error(error as string);
    }
  }

  public static async initClient(): Promise<Client> {
    if (!this.isInitialized) {
      throw new Error("RabbitMQ is not initialized");
    }
    return await Client.initialize(Rabbit.connection);
  }

  static initServer(queue: string) {
    if (!Rabbit.isInitialized) {
      throw new Error("RabbitMQ is not initialized");
    }

    // consume the rpc queue
    this.consumeServerChannel.consume(queue, async (msg) => {
      if (!msg) return;
      let result: ResponseMessage;
      const { contentType, correlationId, replyTo } = msg.properties;
      switch (contentType) {
        case ContentType.JSON:
          const message = JSON.parse(msg.content.toString());
          result = this.performOperation(message);
          break;
        default:
          result = {
            result: null,
            error: `Unsupported content type: ${contentType}`,
          };
          break;
      }
      // send the result back to the client
      this.publishServerChannel.sendToQueue(
        replyTo,
        Buffer.from(JSON.stringify(result)),
        {
          correlationId,
          contentType: ContentType.JSON,
        }
      );
    });
  }

  private static performOperation(message: RequestMessage): ResponseMessage {
    let result: number;
    switch (message.action) {
      case Action.ADD:
        result = message.num1 + message.num2;
        break;
      case Action.SUBTRACT:
        result = message.num1 - message.num2;
        break;
      case Action.MULTIPLY:
        result = message.num1 * message.num2;
        break;
      case Action.DIVIDE:
        if (message.num2 == 0)
          return { result: null, error: "Cannot divide by zero" };
        result = message.num1 / message.num2;
        break;
    }
    return { result };
  }
}

export default Rabbit;
export { ContentType, type RequestMessage, type ResponseMessage };
