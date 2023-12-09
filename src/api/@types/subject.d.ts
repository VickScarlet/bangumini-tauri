declare namespace bgm.subject {
    declare namespace anime {
        interface Episode {
            ep: string
            id: string
            comment: number
            state?: 'Watched' | 'Queue' | 'Drop'
            title?: string
            title_cn?: string
            on_air?: string
            duration?: string
        }

        interface TypedEpisode {
            def: boolean
            typed?: string
            eps: Episode[]
        }

        interface Subject {
            id: string
            img: string
            name_cn: string
            name: string
            hot: number
            eps: TypedEpisode[]
        }
    }
    type Anime = anime.Subject
}

declare namespace bgm {
    declare namespace index {
        interface GuestResult {
            login: false
            id: null
            data: {}
        }

        interface UserResult {
            login: true
            id: string
            data: {
                anime: subject.Anime[]
            }
        }

        type Result = GuestResult | UserResult
    }
}
