# save this as train_iris_onnx.py
import os
from sklearn.datasets import load_iris
from sklearn.tree import DecisionTreeClassifier
import skl2onnx
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType

# Ensure models directory exists
os.makedirs("models", exist_ok=True)

# Load dataset
iris = load_iris()
X, y = iris.data, iris.target

# Train Decision Tree
clf = DecisionTreeClassifier(max_depth=3)
clf.fit(X, y)

# Define input type for ONNX
initial_type = [('float_input', FloatTensorType([None, X.shape[1]]))]

# Convert to ONNX with opset 11 (compatible with ONNX Runtime 1.8)
onnx_model = convert_sklearn(clf, initial_types=initial_type, target_opset=11)

# Save ONNX model in 'models' folder
onnx_file_path = "models/iris_tree_copy.onnx"
with open(onnx_file_path, "wb") as f:
    f.write(onnx_model.SerializeToString())

print(f"ONNX model saved at {onnx_file_path}")
