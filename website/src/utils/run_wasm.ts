export async function run_wasm(args: {
    wasm: ArrayBuffer,
    fun: string,
    args: number[],
}): Promise<number> {
    const mod = await WebAssembly.instantiate(args.wasm);
    try {
        // @ts-expect-error
        return mod.instance.exports[args.fun](...args.args);
    } catch {
        return -1;
    }
}