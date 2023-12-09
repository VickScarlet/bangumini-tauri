export default {
    name: 'former_name',
    indexes: [
        {
            name: '_user_tml',
            keyPath: ['user', 'tml'],
            options: { unique: true },
        },
    ],
}
