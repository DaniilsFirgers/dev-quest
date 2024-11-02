type Config = {
  server: {
    port: number;
    host: string;
  };
  rabbitMq: {
    protocol: string;
    port: number;
    host: string;
    rpc: {
      exchange: string;
      queue: string;
      routingKey: string;
      messageTimeout?: number;
    };
  };
};

const config: Config = {
  server: {
    port: 3000,
    host: "localhost",
  },
  rabbitMq: {
    protocol: "amqp",
    port: 5672,
    host: "localhost",
    rpc: {
      exchange: "rpc",
      queue: "rpc_queue",
      routingKey: "rpc",
      messageTimeout: 5000,
    },
  },
};

export default config;
