import { useState, useEffect, ImgHTMLAttributes } from "react"
import Fallback from '@/assets/b38.svg'

const load = (src: string) => new Promise<boolean>((resolve) => {
    const img = new Image()
    img.onload = () => resolve(true)
    img.onerror = () => resolve(false)
    img.src = src
})

interface Attributes extends ImgHTMLAttributes<HTMLImageElement> {
    fallback?: string
}

export default (prop: Attributes)=>{
    const [loading, setLoading] = useState(true)
    const [last, setLast] = useState('')
    const {
        src,
        fallback,
        ...props
    } = prop

    useEffect(()=>{
        if(!src || src === last) return
        console.debug('loading', src)
        setLoading(true)
        setLast(src)
        load(src).then((loaded)=>setLoading(!loaded))
    },[src])

    return <img
        src={loading ? fallback??Fallback : prop.src}
        {...props}
    />
}
