import { useState, useEffect } from "react"
import Fallback from '@/assets/b38.svg'
import Loading from "./Loading"

const load = (src: string) => new Promise<boolean>((resolve) => {
    const img = new Image()
    img.onload = () => resolve(true)
    img.onerror = () => resolve(false)
    img.src = src
})

interface Attributes extends React.ImgHTMLAttributes<HTMLImageElement> {
    width?: number|string
    height?: number|string
    loadingComponent?: string | React.ReactNode
    fallbackComponent?: string | React.ReactNode
}

export default (prop: Attributes)=>{
    const [loaded, setLoaded] = useState(false)
    const [failed, setFailed] = useState(false)
    const [opacity, setOpacity] = useState(0)
    const [last, setLast] = useState('/')
    const {
        src, loadingComponent, fallbackComponent, style,
        width, height, ...props
    } = prop

    useEffect(()=>{
        if(src === last) return
        setOpacity(0)
        setLoaded(false)
        setOpacity(1)
        setLast(src??'')
        load(src??'').then((loaded)=>{
            setLoaded(true)
            setFailed(!loaded)
        })
    },[src])

    const img = (()=>{
        if(!loaded) return loadingComponent ?? <Loading />
        if(failed) return fallbackComponent ?? Fallback
        return prop.src
    })();

    return <div style={{
        width, height,
        display: 'inline-block',
        ...style
    }}>{typeof img === 'string'
        ? <img src={img} {...props}
            style={{...style,
                opacity,
                width:'100%',
                height:'100%',
                display: 'inline-block',
                position: 'relative',
                transition: 'opacity 0.2s',
            }}
        />
        : <div {...props}
            style={{...style,
                opacity,
                width:'100%',
                height:'100%',
                display: 'inline-block',
                position: 'relative',
                transition: 'opacity 0.2s',
            }}
        >{img}</div>}
    </div>
}
