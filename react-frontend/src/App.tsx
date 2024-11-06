import './App.css'
import init, { transform_image } from '../../pkg/holomorphic_tinkering.js'
import { useEffect, useState } from 'react'

function App() {
  const [file, setFile] = useState<File | null>(null);

  useEffect(() => {
    init().then(() => {
    })
  });

  return (
    <main>
      <input type="file" />
      Hello world
    </main>
  )
}

export default App
