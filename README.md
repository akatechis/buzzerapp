# BuzzerApp

## Overview
This application provides a service for teams looking to host game events,
where a buzzer mechanism is desirable, such as trivia competitions.

Currently, only the buzzer mechanism is provided, allowing participants
to enroll in a competition, and subsequently "buzz in" to attempt to answer
a challenge. There is no scoring or persistence mechanism.

## Protocol
The simple protocol is implemented as follows.

1. All messages are sent as JSON encoded objects and have a `type` property
indicating the type of the message.
