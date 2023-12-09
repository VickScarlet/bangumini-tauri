declare namespace bgm.auth {
    declare namespace signup {
        type CaptchaBuffer = number[]
        interface Result {
            success: boolean
            message?: string
        }
        interface Params {
            formhash: string
            email: string
            password: string
            captcha: string
        }
    }

    declare namespace logout {
        type Result = void
    }
}
