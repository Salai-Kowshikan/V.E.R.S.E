import json
from sklearn.datasets import load_iris
from sklearn.tree import DecisionTreeClassifier

X, y = load_iris(return_X_y=True)
clf = DecisionTreeClassifier(max_depth=3, random_state=42)
clf.fit(X, y)

# Traverse the tree manually
tree_ = clf.tree_
nodes = []
for i in range(tree_.node_count):
    node = {
        "id": i,
        "feature": int(tree_.feature[i]) if tree_.feature[i] != -2 else None,
        "threshold": float(tree_.threshold[i]) if tree_.feature[i] != -2 else None,
        "left": int(tree_.children_left[i]) if tree_.children_left[i] != -1 else None,
        "right": int(tree_.children_right[i]) if tree_.children_right[i] != -1 else None,
        "value": tree_.value[i].tolist()  # leaf values
    }
    nodes.append(node)

with open("tree.json", "w") as f:
    json.dump(nodes, f, indent=4)
print("✅ Tree saved to tree.json")
