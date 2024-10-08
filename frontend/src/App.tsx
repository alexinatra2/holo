import { ChangeEventHandler, FormEventHandler, useCallback, useEffect, useState } from 'react';
import init, { transform_image_wasm } from './wasm'

function App() {
  const [file, setFile] = useState<File | null>(null);
  const [coefficients, setCoefficients] = useState("");
  const [transformedImage, setTransformedImage] = useState<string | null>(null);

  useEffect(() => {
    init();
  }, []);

  const handleSubmit: FormEventHandler<HTMLFormElement> = async (event) => {
    event.preventDefault();
    if (file) {
      const fileArrayBuffer = await file.arrayBuffer();
      const coefficientsArray = coefficients.split(',').map(Number);

      const result = transform_image_wasm(new Uint8Array(fileArrayBuffer), 1980, 1080, new Float64Array(coefficientsArray));
      const transformedBlob = new Blob([result], { type: 'image/jpeg' });
      const imageUrl = URL.createObjectURL(transformedBlob);
      setTransformedImage(imageUrl);
    }
  };

  const handleChangeFile: ChangeEventHandler<HTMLInputElement> = useCallback((event) => {
    if (event.target.files !== null) {
      setFile(event.target.files[0]);
    }
  }, []);

  return (
    <form onSubmit={handleSubmit}>
      <input type="file" onChange={handleChangeFile} />
      <input type="text" value={coefficients} onChange={(e) => setCoefficients(e.target.value)} />
      <button type="submit">Transform</button>
      {transformedImage && <img src={transformedImage} alt="Transformed" />}
    </form>
  );
}

export default App;
