import json
from sklearn.datasets import load_iris
from sklearn.tree import DecisionTreeClassifier
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType


X, y = load_iris(return_X_y=True)
clf = DecisionTreeClassifier(max_depth=5, random_state=42)
clf.fit(X, y)

#save onnx model
initial_type = [("float_input", FloatTensorType([None, X.shape[1]]))]
onnx_model = convert_sklearn(clf, initial_types=initial_type)
with open("iris_tree.onnx", "wb") as f:
    f.write(onnx_model.SerializeToString())

# Traverse the tree manually
tree_ = clf.tree_
nodes = []
print(tree_.node_count)
for i in range(tree_.node_count):
    node = {
        "id": i,
        "feature": int(tree_.feature[i]) if tree_.feature[i] != -2 else None,
        "threshold": float(tree_.threshold[i]) if tree_.feature[i] != -2 else None,
        "left": int(tree_.children_left[i]) if tree_.children_left[i] != -1 else None,
        "right": int(tree_.children_right[i]) if tree_.children_right[i] != -1 else None,
        "value": tree_.value[i].tolist()  
    }
    nodes.append(node)

with open("tree.json", "w") as f:
    json.dump(nodes, f, indent=4)
print("✅ Tree saved to tree.json")

