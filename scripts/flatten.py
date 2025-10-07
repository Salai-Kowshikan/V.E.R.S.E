import onnx
import struct

# -------------------------
# Rust Node layout
# #[repr(C)]
# pub struct Node {
#     feature_index: u32,
#     threshold: f32,
#     left: i32,
#     right: i32,
#     class_label: i32,  // -1 for internal nodes
# }
# -------------------------

# Load ONNX model
model = onnx.load("iris_tree.onnx")
tree_node = model.graph.node[0]  # TreeEnsembleClassifier node

# Helpers to extract attributes by name
def get_int_attr(node, name):
    for attr in node.attribute:
        if attr.name == name:
            return list(attr.ints)
    return []

def get_float_attr(node, name):
    for attr in node.attribute:
        if attr.name == name:
            return list(attr.floats)
    return []

def get_string_attr(node, name):
    for attr in node.attribute:
        if attr.name == name:
            return [s.decode("utf-8") for s in attr.strings]
    return []

# Extract tree info
nodes_nodeids = get_int_attr(tree_node, "nodes_nodeids")
nodes_featureids = get_int_attr(tree_node, "nodes_featureids")
nodes_values = get_float_attr(tree_node, "nodes_values")
nodes_modes = get_string_attr(tree_node, "nodes_modes")

class_nodeids = get_int_attr(tree_node, "class_nodeids")
class_ids = get_int_attr(tree_node, "class_ids")

# Fallback if nodes_modes is empty
if not nodes_modes:
    nodes_modes = ["LEAF" if n in class_nodeids else "BRANCH_LEQ" for n in nodes_nodeids]

# Build node dictionary
node_dict = {}
for i, node_id in enumerate(nodes_nodeids):
    mode = nodes_modes[i]
    if mode == "LEAF":
        # Leaf node
        # Find class label
        idx = class_nodeids.index(node_id)
        class_label = class_ids[idx]
        node_dict[node_id] = {
            "feature_index": 0,
            "threshold": 0.0,
            "left": -1,
            "right": -1,
            "class_label": class_label
        }
    else:
        # Internal node
        node_dict[node_id] = {
            "feature_index": nodes_featureids[i],
            "threshold": nodes_values[i],
            "left": -1,   # will be filled later
            "right": -1,  # will be filled later
            "class_label": -1
        }

# Assign left/right indices (simple DFS ordering)
sorted_node_ids = sorted(node_dict.keys())
for idx, node_id in enumerate(sorted_node_ids):
    node = node_dict[node_id]
    if node["class_label"] == -1:
        # Left child = next node in sorted order
        node["left"] = sorted_node_ids[idx + 1] if idx + 1 < len(sorted_node_ids) else -1
        # Right child = next node not in left subtree (simple approximation)
        node["right"] = sorted_node_ids[idx + 2] if idx + 2 < len(sorted_node_ids) else -1

# Flatten to list
rust_nodes = [node_dict[nid] for nid in sorted_node_ids]

# Write Rust-compatible binary file
with open("iris_tree_nodes.bin", "wb") as f:
    for n in rust_nodes:
        # 'I' = u32, 'f' = f32, 'i' = i32
        f.write(struct.pack('Ifiii',
                            n["feature_index"],
                            n["threshold"],
                            n["left"],
                            n["right"],
                            n["class_label"]))

print(f"Wrote {len(rust_nodes)} nodes to iris_tree_nodes.bin")
