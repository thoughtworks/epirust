module.exports = {
    //SERVER Config
    PORT: process.env.PORT || 3000,

    // DB Config
    DATABASE_URL: process.env.DATABASE_URL || 'mongodb://127.0.0.1/local_database',

    // Kafka Config
    KAFKA_URL: process.env.KAFKA_URL || 'localhost:9092',
    COUNTS_TOPIC: 'counts_updated',
    GRID_MESSAGE_TOPIC: 'citizen_states_updated',
    KAFKA_GROUP: process.env.KAFKA_GROUP || 'dev_server_consumer',

    //Client Config
    CLIENT_URL: process.env.CLIENT_URL || 'http://localhost:3001'

};
