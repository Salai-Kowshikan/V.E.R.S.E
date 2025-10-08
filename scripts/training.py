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

# from sklearn.datasets import load_iris
# from sklearn.tree import DecisionTreeClassifier
# from skl2onnx import convert_sklearn
# from skl2onnx.common.data_types import FloatTensorType
# import struct

# # -----------------------------
# # Step 1: Train Decision Tree
# # -----------------------------
# iris = load_iris()
# X, y = iris.data, iris.target

# model = DecisionTreeClassifier(max_depth=3)
# model.fit(X, y)

# # -----------------------------
# # Step 2: Convert to ONNX
# # -----------------------------
# initial_type = [('float_input', FloatTensorType([None, 4]))]
# onnx_model = convert_sklearn(model, initial_types=initial_type)
# with open("iris_tree.onnx", "wb") as f:
#     f.write(onnx_model.SerializeToString())
# print("ONNX model saved as iris_tree.onnx")

# # -----------------------------
# # Step 3: Serialize tree nodes for Rust guest
# # -----------------------------
# # Node structure in Rust:
# # struct Node { feature_index: u32, threshold: f32, left: i32, right: i32, class_label: i32 }

# tree = model.tree_
# n_nodes = tree.node_count

# # Create a binary .bin file compatible with Rust Node
# with open("iris_tree_nodes.bin", "wb") as f:
#     for i in range(n_nodes):
#         feature_index = tree.feature[i] if tree.feature[i] != -2 else 0  # -2 means leaf in sklearn
#         threshold = tree.threshold[i] if tree.feature[i] != -2 else 0.0
#         left = tree.children_left[i]
#         right = tree.children_right[i]
#         class_label = -1  # non-leaf
#         if left == -1 and right == -1:
#             # Leaf node
#             class_label = int(tree.value[i].argmax())
#             left = -1
#             right = -1
#         # Pack as C-compatible struct: u32, f32, i32, i32, i32
#         f.write(struct.pack("<Ifiii", feature_index, threshold, left, right, class_label))

# print("Binary tree nodes saved as iris_tree_nodes.bin")

