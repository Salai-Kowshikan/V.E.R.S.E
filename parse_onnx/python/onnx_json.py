import onnx
import json

model = onnx.load("iris_tree.onnx")
graph = model.graph

# Locate the tree ensemble
for node in graph.node:
    if node.op_type == "TreeEnsembleClassifier":
        attrs = {a.name: onnx.helper.get_attribute_value(a) for a in node.attribute}
        break

features = attrs["nodes_featureids"]
thresholds = attrs["nodes_values"]
left_children = attrs["nodes_falsenodeids"]
right_children = attrs["nodes_truenodeids"]
modes = attrs["nodes_modes"]
node_ids = attrs["nodes_nodeids"]

# Build leaf value mapping
leaf_values = {}
for nid, cid, w in zip(attrs["class_nodeids"], attrs["class_ids"], attrs["class_weights"]):
    nid = int(nid)
    cid = int(cid)
    w = float(w)
    leaf_values.setdefault(nid, [])
    while len(leaf_values[nid]) <= cid:
        leaf_values[nid].append(0.0)
    leaf_values[nid][cid] = w

# Construct JSON
nodes = []
for i, node_id in enumerate(node_ids):
    mode = modes[i]
    node = {
        "id": int(node_id),
        "feature": int(features[i]) if mode == b"BRANCH_LEQ" else None,
        "threshold": float(thresholds[i]) if mode == b"BRANCH_LEQ" else None,
        "left": int(left_children[i]) if left_children[i] != -1 else None,
        "right": int(right_children[i]) if right_children[i] != -1 else None,
        "value": leaf_values.get(int(node_id), None)
    }
    nodes.append(node)

with open("onnx_json.json", "w") as f:
    json.dump(nodes, f, indent=4)

print("✅ Tree with leaf values saved to onnx_json.json")
