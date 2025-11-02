import json
import numpy as np
from sklearn.datasets import load_iris
from sklearn.tree import DecisionTreeClassifier

# Load data
iris = load_iris()
X, y = iris.data, iris.target

# Train tree
clf = DecisionTreeClassifier(max_depth=3)
clf.fit(X, y)

tree_ = clf.tree_

# Export tree structure + thresholds + leaf outputs
tree_data = {
    "feature": tree_.feature.tolist(),
    "threshold": tree_.threshold.tolist(),
    "children_left": tree_.children_left.tolist(),
    "children_right": tree_.children_right.tolist(),
    "values": [list(v[0]) for v in tree_.value]
}

# Validation dataset (5 rows)
X_val = np.array([
    [5.1, 3.5, 1.4, 0.2],
    [6.2, 2.9, 4.3, 1.3],
    [5.9, 3.0, 5.1, 1.8],
    [5.5, 2.5, 4.0, 1.3],
    [6.3, 3.3, 6.0, 2.5]
])

# Quantize inputs
X_val_scaled = (X_val * 100).astype(int)

json.dump(tree_data, open("tree.json", "w"))
json.dump({"inputs": X_val_scaled.tolist()}, open("validation_inputs.json", "w"))

print("Tree and validation inputs exported")
