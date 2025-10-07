import struct

# Define your Rust struct layout
NODE_STRUCT = struct.Struct('Ifiii')  # u32, f32, i32, i32, i32

nodes = []

with open("iris_tree_nodes.bin", "rb") as f:
    while chunk := f.read(NODE_STRUCT.size):
        feature_index, threshold, left, right, class_label = NODE_STRUCT.unpack(chunk)
        nodes.append({
            "feature_index": feature_index,
            "threshold": threshold,
            "left": left,
            "right": right,
            "class_label": class_label
        })

# Print all nodes
for i, node in enumerate(nodes):
    print(f"Node {i}: {node}")
