import onnx
import json

model = onnx.load("iris_tree_copy.onnx")

# Get the tree node
tree_node = next(
    n for n in model.graph.node if n.op_type in ["TreeEnsembleClassifier", "TreeEnsembleRegressor"]
)

# Extract attributes
def get_attr(node, name):
    for attr in node.attribute:
        if attr.name == name:
            return attr
    return None

node_ids = list(get_attr(tree_node, "nodes_nodeids").ints)
tree_ids = list(get_attr(tree_node, "nodes_treeids").ints)
features = list(get_attr(tree_node, "nodes_featureids").ints)
thresholds = list(get_attr(tree_node, "nodes_values").floats)
modes = [s.decode() for s in get_attr(tree_node, "nodes_modes").strings]
true_ids = list(get_attr(tree_node, "nodes_truenodeids").ints)
false_ids = list(get_attr(tree_node, "nodes_falsenodeids").ints)
leaf_weights_attr = get_attr(tree_node, "target_weights")
leaf_weights = list(leaf_weights_attr.floats) if leaf_weights_attr else [None] * len(node_ids)

# Build node dictionary
nodes = {}
for i, nid in enumerate(node_ids):
    nodes[nid] = {
        "node_id": nid,
        "feature": features[i],
        "threshold": thresholds[i],
        "mode": modes[i],
        "left": true_ids[i] if true_ids[i] != -1 else None,
        "right": false_ids[i] if false_ids[i] != -1 else None,
        "weight": leaf_weights[i]
    }

# Recursive function to build nested tree
def build_tree(node_id):
    node = nodes[node_id]
    if node["mode"] == "LEAF":
        return {"weight": node["weight"]}
    return {
        "feature": node["feature"],
        "threshold": node["threshold"],
        "left": build_tree(node["left"]) if node["left"] is not None else None,
        "right": build_tree(node["right"]) if node["right"] is not None else None
    }

# Assuming tree_id 0 and root node_id 0
nested_tree = build_tree(0)

# Save JSON
with open("tree.json", "w") as f:
    json.dump(nested_tree, f, indent=4)

print("Nested decision tree saved as tree.json")
