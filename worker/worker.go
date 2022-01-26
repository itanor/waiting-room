package main

import (
  "fmt"
  "log"
  "github.com/streadway/amqp"
  "context"
  "github.com/go-redis/redis/v8"
)
var ctx = context.Background()

func failOnError(err error, msg string) {
  if err != nil {
    log.Fatalf("%s: %s", msg, err)
  }
}

func read_from_redis() {
  rdb := redis.NewClient(&redis.Options{
    Addr:     "localhost:6379",
    Password: "", // no password set
    DB:       0,  // use default DB
  })

  val, err := rdb.Get(ctx, "itanor@gmail.com").Result()
  if err != nil {
      panic(err)
  }
  fmt.Println("key", val)
}

func main() {
  conn, err := amqp.Dial("amqp://guest:guest@localhost:5672/")
  failOnError(err, "Failed to connect to RabbitMQ")
  defer conn.Close()
  fmt.Println("worker...!")

  ch, err := conn.Channel()
  failOnError(err, "Failed to open a channel")
  defer ch.Close()

  q, err := ch.QueueDeclare(
    "generated-pass", // name
    true,   // durable
    false,   // delete when unused
    false,   // exclusive
    false,   // no-wait
    nil,     // arguments
  )
  failOnError(err, "Failed to declare a queue")

  msgs, err := ch.Consume(
    q.Name, // queue
    "",     // consumer
    true,   // auto-ack
    false,  // exclusive
    false,  // no-local
    false,  // no-wait
    nil,    // args
  )

  forever := make(chan bool)

  read_from_redis()

  go func() {
    for d := range msgs {
      log.Printf("Received a message: %s", d.Body)
    }
  }()

  log.Printf(" [*] Waiting for messages. To exit press CTRL+C")
  <-forever
}
