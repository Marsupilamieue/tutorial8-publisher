a. How many data your publlsher program will send to the message broker in one
run?
5 daata
b. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber
program, what does it mean?
So, when the URL "amqp://guest:guest@localhost:5672" is used in both the publisher and subscriber programs, it means that both programs are connecting to the same AMQP broker running on the local machine, using the default guest credentials. This allows them to communicate with each other via the broker, sending and receiving messages.

running RabbitMQ
![running Rabbit MQ](/assets/runrabbit.png)