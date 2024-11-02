import amqp from "amqplib";
import config from "../config.js";
import { v4 as uuidv4 } from "uuid";
import { type ResponseMessage, type RequestMessage } from "./index.js";

class Client {
  static connection: amqp.Connection;
  private consumeClientChannel!: amqp.Channel;
  private publishClientChannel!: amqp.Channel;
  private queue!: string;
  private messageMap: { [key: string]: (value: ResponseMessage) => void } = {};

  private constructor() {}

  static async initialize(connection: amqp.Connection) {
    // intialize the connection only once
    if (!this.connection) this.connection = connection;
    // now define consumer and producer channels

    // NOTE: it is recommended to create a separate channel for producing and consuming
    // each client should have its own channels
    const client = new Client();
    client.consumeClientChannel = await this.connection.createChannel();
    client.publishClientChannel = await this.connection.createChannel();

    // assert the exchange
    // exchange is set to durable so that it can survive a server restart
    // it will be direct exchange type
    await client.consumeClientChannel.assertExchange(
      config.rabbitMq.rpc.exchange,
      "direct",
      {
        durable: true,
      }
    );

    // bind an anonymous queue to consumer channel as we will be listening to answer message from server
    const q = await client.consumeClientChannel.assertQueue("", {
      exclusive: true,
    });

    client.queue = q.queue;
    client.consume();

    return client;
  }

  public async publish(msg: RequestMessage) {
    // publish a message to the rpc exchange
    const correlationId = uuidv4();
    this.publishClientChannel.publish(
      config.rabbitMq.rpc.exchange,
      config.rabbitMq.rpc.routingKey,
      Buffer.from(JSON.stringify(msg)),
      {
        correlationId,
        replyTo: this.queue,
        contentType: "application/json",
      }
    );
    return this.createMessageTimeoutPromise(correlationId);
  }

  private createMessageTimeoutPromise(
    correlationId: string
  ): Promise<ResponseMessage> {
    // resolve the promise with message and reject wiith error message if timeout occurs
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        delete this.messageMap[correlationId];
        const errorMessage: ResponseMessage = {
          error: "Timeout error",
          result: null,
        };
        reject(errorMessage);
      }, config.rabbitMq.rpc.messageTimeout);

      this.messageMap[correlationId] = (value) => {
        clearTimeout(timeout);
        resolve(value);
      };
    });
  }

  private async consume() {
    // consume messages from the anonymous queue
    this.consumeClientChannel.consume(this.queue, (msg) => {
      if (!msg) return;
      const correlationId = msg.properties.correlationId;

      const promiseResolver = this.messageMap[correlationId];
      if (!promiseResolver) return;

      const parsedMsg: ResponseMessage = JSON.parse(msg.content.toString());
      promiseResolver(parsedMsg);

      delete this.messageMap[correlationId];
    });
  }
}

export default Client;
