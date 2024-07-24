import { JSX, createSignal } from 'solid-js'
import { KediIde } from 'src/components/KediIde'
import './index.css'

function App() {
  return (
    <div style={styles.root}>
      <h1>kedi-lang playground</h1>
      <KediIde />
    </div>
  )
}

export default App

const styles: Record<string, JSX.CSSProperties> = {
  root: {
    display: 'flex',
    width: '100%',
    height: '100%',
    'flex-direction': 'column',
  },
}