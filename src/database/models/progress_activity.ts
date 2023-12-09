export default {
    name: 'progress_activity',
    indexes: [
        {
            name: '_user_time',
            keyPath: ['user', 'time'],
            options: { unique: true },
        },
    ],
}
