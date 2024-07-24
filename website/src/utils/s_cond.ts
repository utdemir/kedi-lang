import { Accessor } from "solid-js";

export function s_cond<T, K extends T, Ret,>(
    value: Accessor<T>,
    predicate: (v: T) => v is K,
    whenTrue: (v: K) => Ret,
    whenFalse: (v: Exclude<T, K>) => Ret
): Accessor<Ret> {
    return () => {
        let v = value();
        if (predicate(v)) {
            return whenTrue(v);
        } else {
            return whenFalse(v as Exclude<T, K>);
        }
    }
}


export function s_defined<T, Ret>(
    value: Accessor<T | null | undefined>,
    whenDefined: (v: T) => Ret,
    whenUndefined: () => Ret
): Accessor<Ret> {
    return s_cond(
        value,
        (v) => v !== null && v !== undefined,
        whenDefined,
        whenUndefined
    )
}
