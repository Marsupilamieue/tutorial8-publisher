-  How many data your publlsher program will send to the message broker in one
run?

    - 5 data

-  The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?

    - So, when the URL "amqp://guest:guest@localhost:5672" is used in both the publisher and subscriber programs, it means that both programs are connecting to the same AMQP broker running on the local machine, using the default guest credentials. This allows them to communicate with each other via the broker, sending and receiving messages.

- running RabbitMQ
![running Rabbit MQ](/assets/runrabbit.png)
![running Rabbit MQ](/assets/runconsole.png)
The picture above demonstrates the process initiated by the publisher. Publisher will send data to the message queue. Subscribers linked to this message queue will then retrieve the data and display it in the console based on the implemented code.

![running Rabbit MQ](/assets/spike.png)
From the massage rates graph it can be seen that every time the publisher is run, there will be an increase in the message rate on rabbitMQ.

