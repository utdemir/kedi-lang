import { createCodeMirror, createEditorControlledValue, createEditorReadonly } from "solid-codemirror";
import { Accessor, createEffect, createSignal, JSX } from "solid-js";
import { lineNumbers } from "@codemirror/view";

export function Editor(props: {
    value: Accessor<string>;
    readonly?: boolean;
    onValueChange?: (value: string) => void;
    overrideValue?: string;
    style?: JSX.CSSProperties;
}) {
    const { ref, editorView, createExtension } = createCodeMirror({
        value: props.value(),
        onValueChange: (value) => {
            props.onValueChange?.(value);
        }
    });

    createEditorControlledValue(editorView, props.value);

    const [readonly, _setReadonly] = createSignal(props.readonly ?? false);
    createEditorReadonly(editorView, readonly);

    createExtension(lineNumbers);

    return <div ref={ref} style={{ ...styles.root, ...props.style }} />;
}

const styles: Record<string, JSX.CSSProperties> = {
    root: {
        'background-color': '#f0f0f0',
    }
}