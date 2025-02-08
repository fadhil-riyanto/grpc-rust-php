<?php
require './vendor/autoload.php';
// require './compiled/Helloworld/GreeterClient.php';

function greet($hostname)
{
        $client = new Helloworld\GreeterClient($hostname, [
                'credentials' => Grpc\ChannelCredentials::createInsecure(),
        ]);

        $request = new Helloworld\HelloRequest();
        $request->setName("hi bro");
        list($response, $status) = $client->SayHello($request)->wait();

        if ($status->code !== Grpc\STATUS_OK) {
                echo "ERROR: " . $status->code . ", " . $status->details . PHP_EOL;
                exit(1);
        }
        echo $response->getMessage() . PHP_EOL;
}


greet("127.0.0.1:8998");
