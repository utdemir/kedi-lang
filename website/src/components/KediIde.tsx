import { Accessor, createEffect, createSignal, JSX, Show } from "solid-js";
import * as kedi_lang_web from "assets/generated/compiler_web";
import { Editor } from "./Editor";
import { s_cond } from "src/utils/s_cond";
import { STYLE_VARS } from "src/utils/style_vars";
import { run_wasm } from "src/utils/run_wasm";
import { base64 } from "rfc4648";

import example_fibonacci from '../../../compiler/tests/data/examples/fibonacci.kedi?raw';

await kedi_lang_web.default();

const EMPTY_COMPILE_RESULT: kedi_lang_web.CompileResultWeb = kedi_lang_web.runner("") as kedi_lang_web.CompileResultWeb;

export function KediIde() {
    const [source, setSource] = createSignal(example_fibonacci);
    const [result, setResult] = createSignal<kedi_lang_web.CompileResultWeb>(EMPTY_COMPILE_RESULT);

    const possibleInternalType = [
        { label: 'Syntax', value: 'syntax' },
        { label: 'Plain', value: 'plain' },
        { label: 'Simple', value: 'simple' },
        { label: 'Wasm', value: 'wasm' }
    ] as const;

    const [internalType, setInternalType] = createSignal<
        null | typeof possibleInternalType[number]['value']
    >(null);

    createEffect(() => {
        const output = kedi_lang_web.runner(source()) as kedi_lang_web.CompileResultWeb;
        setResult(output);
    });

    const [executionId, setExecutionId] = createSignal(0);
    const [executionResult, setExecutionResult] = createSignal<string | null>(null);
    createEffect(() => {
        const wasm_b64 = s_success(result, (r) => r.wasm, (e) => null)();
        if (!wasm_b64) return;
        const eid = setExecutionId((eid) => eid + 1);

        const wasm = base64.parse(wasm_b64);

        (window as any).debug = wasm;
        run_wasm({
            wasm,
            fun: 'main',
            args: []
        }).then((res) => {
            if (eid !== executionId()) return;
            setExecutionResult(String(res));
        })
    });

    return (
        <div style={styles.root}>
            <div style={styles.toolbar}>
                <select onChange={(e) => setInternalType(
                    e.currentTarget.value === "" ? null :
                        e.currentTarget.value as any
                )}>
                    <option value={""}>Select Internal Type</option>
                    {possibleInternalType.map((t) => (
                        <option value={t.value}>{t.label}</option>
                    ))}
                </select>
            </div>
            <div style={styles.editors}>
                <div style={{ ...styles.editor_left, }}>
                    <Editor onValueChange={setSource} value={source} style={styles.editor} />
                </div>

                <div style={{ ...styles.editor_right, }}>
                    <Show when={internalType() !== null}>
                        <Editor readonly style={styles.editor} value={
                            s_success(
                                result,
                                (r) =>
                                    internalType() === 'syntax' ? r.syntax :
                                        internalType() === 'plain' ? r.plain :
                                            internalType() === 'simple' ? r.simple :
                                                internalType() === 'wasm' ? r.wat :
                                                    '',
                                (err) => err.message
                            )
                        } />
                    </Show>
                    <div>
                        Output: {executionResult()}
                    </div>
                </div>
            </div>
        </div >
    );
}

const styles: Record<string, JSX.CSSProperties> = {
    root: {
        display: 'flex',
        'flex-direction': 'column',
    },
    toolbar: {
        display: 'flex',
        'flex-direction': 'row-reverse',
        gap: '10px',
    },
    editors: {
        display: 'flex',
        'flex-direction': 'row',
        gap: '10px',
    },
    editor: {
        'background-color': STYLE_VARS['color2']
    },
    editor_left: {
        width: '50%',
    },
    editor_right: {
        width: '50%',
    },
    source_editor: {
        width: '100%',
        height: '100%',

        'background-color': '#3a3a3a',
    },
    output_textarea: {
        width: '100%',
        height: '100%',
    },
}

function s_success<K>(
    value: Accessor<kedi_lang_web.CompileResultWeb>,
    whenSuccess: (v: kedi_lang_web.CompileSuccessWeb) => K,
    whenError: (v: kedi_lang_web.CompileErrorWeb) => K
): Accessor<K> {
    return s_cond(
        value,
        (v) => 'Success' in v,
        (v) => whenSuccess(v.Success),
        (v) => whenError(v.Error)
    )
}
