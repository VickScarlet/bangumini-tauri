export const getImageUriFromArray = (array: Array<number>) => {
    const blob = new Blob([new Uint8Array(array)], { type: 'image/jpeg' })
    return URL.createObjectURL(blob)
}
