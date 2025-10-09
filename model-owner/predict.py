import onnxruntime as ort
import numpy as np

session = ort.InferenceSession("iris_tree_copy.onnx")

X_test = np.array([
    [5.1, 3.5, 1.4, 0.2],
    [7.0, 3.2, 4.7, 1.4],
    [6.3, 3.3, 6.0, 2.5],
    [5.8, 2.7, 5.1, 1.9],
    [5.0, 3.4, 1.5, 0.2],
], dtype=np.float32)

y_true = np.array([0, 1, 2, 2, 0])

input_name = session.get_inputs()[0].name

outputs = session.run(None, {input_name: X_test})
y_pred = outputs[0].ravel().astype(int)  

correct = np.sum(y_true == y_pred)
accuracy = correct / len(y_true) * 100

print("=== Prediction Comparison ===")
for i, (pred, actual) in enumerate(zip(y_pred, y_true)):
    status = "✅ Correct" if pred == actual else "❌ Wrong"
    print(f"Sample {i+1}: Predicted = {pred}, Actual = {actual} → {status}")

print(f"\nOverall Accuracy: {accuracy:.2f}%")