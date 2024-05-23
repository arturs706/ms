import { createSignal, Suspense, createResource } from "solid-js";
import styles from "../css/lettings.module.css";
import Animation from "../components/Animation";

export default function Lettings() {
  const [selectedImageFiles, setSelectedImageFiles] = createSignal<File[]>([]);
  const [imagePreviews, setImagePreviews] = createSignal<string[]>([]);
  const [uploading, setUploading] = createSignal(false);
  const [finished, setFinished] = createSignal(false);

  const handleFileSelect = (event: Event) => {
    const files = (event.target as HTMLInputElement).files;
    if (files) {
      const imageFiles = Array.from(files).filter((file) =>
        ['image/png', 'image/jpeg', 'image/avif', 'image/jp2', 'image/webp'].includes(file.type)
      );
      setSelectedImageFiles((prevFiles) => [...prevFiles, ...imageFiles]);

      const previews = Array.from(files).map(file => URL.createObjectURL(file));
      setImagePreviews((prevPreviews) => [...prevPreviews, ...previews]);
    }     
  };

  const removeImage = (index: number) => {
    const files = [...selectedImageFiles()];
    files.splice(index, 1);
    setSelectedImageFiles(files);

    const previews = [...imagePreviews()];
    previews.splice(index, 1);
    setImagePreviews(previews);
  };

  const uploadImages = () => {
    const formData = new FormData();
    selectedImageFiles().forEach(file => {
      formData.append("images", file);
    });

    setUploading(true); // Set uploading to true when starting upload
  
    fetch("http://localhost:10003/api/v1/properties/uploadimages/550e8400-e29b-41d4-a716-446655440000", {
      method: "POST",
      body: formData,
      headers: {
        "Authorization": "Bearer " + "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJBY2Nlc3MiLCJleHAiOjE3MTA0NDU1NDAsImlhdCI6MTcxMDQ0NDY0MCwicm9sZSI6IlVzZXIifQ.Fvbm-C7T_lqP2Xx0Yk1EkODER536P44aVWtPRzaWTec",
        "x-image-quantity": selectedImageFiles().length.toString(),
      },
      
      
    })
    .then(response => {
      setUploading(false); // Set uploading to false when upload completes
      if (response.ok) {
        console.log("Images uploaded successfully");
        setFinished(true);
      } else {
        console.error(response);
        console.error("Error uploading images");
        setFinished(false);
      }
    })
    .catch(error => {
      console.error(error);

      setUploading(false); // Set uploading to false in case of error
      console.error("Error uploading images:", error);
      setFinished(false);
    });
  };

  return (
    <main class={styles.main}>
      <div>
      </div>
      {finished() ? (
        <div>Images uploaded successfully</div>
      ) : 
      (uploading() && ! finished()) ? (
        <div>
          <Animation />
          <div class="loader">Uploading...</div>
        </div>
      ) : (
        <Suspense fallback={<div class="loader">Loading...</div>}>
        <label class={styles.customfileupload}>
        <input type="file" accept=".png,.jpg,.jpeg,.avif,.jp2,.webp" multiple onChange={handleFileSelect}/>
        ADD IMAGES
      </label>
        {imagePreviews().map((preview, index) => (
          <div>
            <img src={preview} alt="" style={{ width: "100px", height: "100px", margin: "5px" }} />
            <button onClick={() => removeImage(index)}>Remove</button>
          </div>
        ))}
        {selectedImageFiles().length > 0 && (
          <button onClick={uploadImages}>Upload</button>
        )}
        </Suspense>
      )}
    </main>
  );
}
