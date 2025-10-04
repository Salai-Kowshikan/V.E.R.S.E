from sklearn.datasets import load_iris
from sklearn.tree import DecisionTreeClassifier
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType

# Load Iris dataset
iris = load_iris()
X, y = iris.data, iris.target

# Train Decision Tree
model = DecisionTreeClassifier(max_depth=3)
model.fit(X, y)

# Convert to ONNX
initial_type = [('float_input', FloatTensorType([None, 4]))]
onnx_model = convert_sklearn(model, initial_types=initial_type)

# Save ONNX file
with open("iris_tree.onnx", "wb") as f:
    f.write(onnx_model.SerializeToString())

print("ONNX model saved as iris_tree.onnx")
