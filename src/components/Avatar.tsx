import Image from './Image';
import Fallback from '@/assets/avatar-fallback.jpg';

interface Attributes extends React.ImgHTMLAttributes<HTMLImageElement> {
    uid: number|string;
    larger?: boolean;
}

const getSrc = (nid:number, larger=false)=>{
    let s = larger?'c':'l'
    let a = (''+Math.floor(nid/1000000)).padStart(3, '0');
    let b = (''+Math.floor(nid/10000)%100).padStart(2, '0');
    let c = (''+Math.floor(nid/100)%100).padStart(2, '0');
    return `https://lain.bgm.tv/pic/user/${s}/${a}/${b}/${c}/${nid}.jpg?hd=1`;
}

export default (prop: Attributes)=>{
    const { uid, larger, ...props } = prop;
    let nid = Number(uid);
    if(!isNaN(nid))
        return <Image src={getSrc(nid, larger)} loadingComponent={Fallback} fallbackComponent={Fallback} {...props} />
    else
        return <Image src={Fallback} loadingComponent={Fallback} fallbackComponent={Fallback} {...props} />
}