# MarketBeacon: Real-Time Crypto Price Deviation Alerts

A serverless, TA-driven system for personalized cryptocurrency price monitoring via WebSocket streams.

MarketBeacon is a powerful tool designed for cryptocurrency traders and developers seeking to automate their market monitoring and trading strategies. It provides real-time alerts based on user-defined technical analysis (TA) conditions, delivered directly through WebSocket streams. Unlike conventional alert systems that rely on polling APIs, MarketBeacon leverages efficient WebSocket data feeds from exchanges, minimizing latency and ensuring timely notifications. This allows users to react swiftly to market movements and capitalize on emerging opportunities.

The core functionality of MarketBeacon lies in its ability to execute user-defined condition sets, written as serverless functions, against incoming price data. These functions can incorporate any combination of technical indicators, price levels, and custom logic, enabling highly personalized and sophisticated alert triggers. The system is designed for scalability and reliability, using a serverless architecture to handle fluctuating data volumes and ensure continuous operation. This eliminates the need for managing complex infrastructure, allowing users to focus on refining their trading strategies.

MarketBeacon empowers users to define their own trading rules and receive alerts tailored to their specific needs. By combining real-time WebSocket data, customizable TA conditions, and a serverless execution environment, MarketBeacon offers a flexible and efficient solution for proactive cryptocurrency market monitoring. The system is designed to be easily integrated into existing trading platforms or used as a standalone tool for informed decision-making.

Key Features:

*   **Real-time WebSocket Data Ingestion:** Utilizes asynchronous Rust code with `tokio` to establish and maintain persistent WebSocket connections to cryptocurrency exchanges, minimizing latency. Raw data is parsed using `serde_json` and validated against a predefined schema.
*   **Serverless Condition Execution:** User-defined condition sets are deployed as AWS Lambda functions (or similar serverless platform). These functions, written in a supported language (e.g., JavaScript, Python), receive real-time price data and return a boolean value indicating whether the alert should be triggered. The system uses `aws_lambda_events` crate for handling Lambda invocation events.
*   **TA-Driven Alert Triggers:** Supports a wide range of technical indicators (e.g., moving averages, RSI, MACD) calculated using a dedicated TA library (e.g., `ta-rs`). Users can combine these indicators with price levels and custom logic to create complex alert conditions.
*   **User-Defined Alert Configuration:** Allows users to define alert parameters, including cryptocurrency pairs, exchanges, technical indicators, trigger conditions, and notification endpoints. Configuration is managed through a centralized database (e.g., PostgreSQL) accessed via `diesel.rs`.
*   **Scalable Serverless Architecture:** Deployed on a serverless platform (e.g., AWS Lambda, Google Cloud Functions) to handle fluctuating data volumes and ensure continuous operation. This eliminates the need for managing complex infrastructure.
*   **WebSocket Alert Delivery:** Delivers real-time alerts directly to users through WebSocket connections managed by an API Gateway service (e.g., AWS API Gateway). The WebSocket endpoint provides a persistent connection for receiving alerts.
*   **Secure Authentication and Authorization:** Implements robust authentication and authorization mechanisms to protect user data and prevent unauthorized access. Uses JWT tokens for authentication and role-based access control (RBAC).

Technology Stack:

*   **Rust:** Programming language for the core data ingestion, processing, and alert delivery components. Leverages Rust's performance, memory safety, and concurrency features.
*   **Tokio:** Asynchronous runtime for managing WebSocket connections and concurrent tasks.
*   **Serde:** Library for serializing and deserializing data, used for parsing WebSocket data and communicating with Lambda functions.
*   **Diesel.rs:** ORM (Object-Relational Mapper) for interacting with the database storing user configurations and alert history.
*   **AWS Lambda (or similar):** Serverless platform for executing user-defined condition sets.
*   **AWS API Gateway (or similar):** Manages WebSocket connections for alert delivery.
*   **PostgreSQL (or similar):** Database for storing user configurations, alert history, and other persistent data.

Installation:

1.  Clone the repository:
    git clone https://github.com/uhsr/MarketBeacon.git
    cd MarketBeacon

2.  Install Rust:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3.  Install Diesel CLI:
    cargo install diesel_cli --no-default-features --features postgres

4.  Install necessary system dependencies for PostgreSQL:
    (Example for Debian/Ubuntu: sudo apt-get update && sudo apt-get install libpq-dev)

5.  Create a PostgreSQL database:
    (Instructions depend on your PostgreSQL setup. Example: `createdb marketbeacon`)

6.  Configure the database connection URL in `.env` (see Configuration section).

7.  Run database migrations:
    diesel migration run

8.  Build the project:
    cargo build --release

Configuration:

MarketBeacon requires the following environment variables to be configured:

*   `DATABASE_URL`: The connection string for your PostgreSQL database. Example: `postgres://user:password@host:port/database`
*   `AWS_REGION`: The AWS region where your Lambda functions are deployed (if using AWS Lambda). Example: `us-east-1`
*   `API_GATEWAY_URL`: The URL of your API Gateway for WebSocket connections (if using API Gateway). Example: `wss://your-api-gateway-url.execute-api.us-east-1.amazonaws.com/prod`
*   `LAMBDA_FUNCTION_NAME`: The name of your Lambda function for executing condition sets.
*   `JWT_SECRET`: A secret key used for signing JWT tokens for authentication. Generate a strong, random key.

These environment variables can be set in a `.env` file in the root directory of the project. The `.env` file should not be committed to version control.

Usage:

After installing and configuring MarketBeacon, you can run the core service:

target/release/marketbeacon

This will start the WebSocket data ingestion and alert delivery processes.

To define alert conditions, you need to deploy a Lambda function that implements your desired logic. The Lambda function should accept a JSON payload containing real-time price data and return a boolean value indicating whether the alert should be triggered. Example Python Lambda function:

import json

def lambda_handler(event, context):
    data = json.loads(event['body'])
    price = data['price']
    if price > 10000:
        return {
            'statusCode': 200,
            'body': json.dumps({'trigger': True})
        }
    else:
        return {
            'statusCode': 200,
            'body': json.dumps({'trigger': False})
        }

Users can interact with the system through the WebSocket API Gateway endpoint to subscribe to specific cryptocurrency pairs and receive real-time alerts. Detailed API documentation will be provided separately.

Contributing:

We welcome contributions to MarketBeacon! Please follow these guidelines:

*   Fork the repository.
*   Create a new branch for your feature or bug fix.
*   Write clear and concise commit messages.
*   Submit a pull request with a detailed description of your changes.
*   Ensure that your code adheres to the Rust coding style guidelines.
*   Write unit tests for your changes.

License:

This project is licensed under the MIT License. See the [LICENSE](https://github.com/uhsr/MarketBeacon/blob/main/LICENSE) file for details.

Acknowledgements:

We would like to thank the Rust community and the developers of the open-source libraries used in this project.