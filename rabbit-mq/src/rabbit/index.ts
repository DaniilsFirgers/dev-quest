import amqp from "amqplib";
import config from "../config.js";
import Client from "./client.js";
import {
  Action,
  ContentType,
  RequestMessage,
  ResponseMessage,
  Topics,
} from "./types.js";
import { TOPICS } from "./utils.js";

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
        "topic",
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
        `${config.rabbitMq.rpc.routingKey}.*` // we will be listening to all topics with the prefix rpc;
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
      const routingKeyOperation = msg.fields.routingKey as Topics;
      if (!TOPICS[routingKeyOperation]) {
        result = {
          result: null,
          error: `Unsupported operation: ${routingKeyOperation}`,
        };
      } else {
        switch (contentType) {
          case ContentType.JSON:
            const message: RequestMessage = JSON.parse(msg.content.toString());
            result = TOPICS[routingKeyOperation](message);
            break;
          default:
            result = {
              result: null,
              error: `Unsupported content type: ${contentType}`,
            };
            break;
        }
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
}

export default Rabbit;
export { ContentType, type RequestMessage, type ResponseMessage };
