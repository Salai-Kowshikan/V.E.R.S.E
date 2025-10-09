import struct
import graphviz

# Match the Rust struct layout
class Node:
    def __init__(self, feature_index, threshold, left, right, class_label):
        self.feature_index = feature_index
        self.threshold = threshold
        self.left = left
        self.right = right
        self.class_label = class_label

def read_nodes_from_bin(filename):
    nodes = []
    struct_fmt = "i f i i i"  # i32, f32, i32, i32, i32
    node_size = struct.calcsize(struct_fmt)

    with open(filename, "rb") as f:
        data = f.read()

    for i in range(0, len(data), node_size):
        feature, thresh, left, right, label = struct.unpack_from(struct_fmt, data, i)
        nodes.append(Node(feature, thresh, left, right, label))
    return nodes

def build_graphviz_tree(nodes, output_name="bin_tree"):
    dot = graphviz.Digraph(format="pdf")
    for i, node in enumerate(nodes):
        # Leaf node
        if node.left == -1 and node.right == -1:
            dot.node(str(i), f"Class = {node.class_label}",
                     shape="box", style="filled", color="lightblue")
        # Internal node
        else:
            dot.node(str(i), f"X[{node.feature_index}] <= {node.threshold:.2f}")
            if 0 <= node.left < len(nodes):
                dot.edge(str(i), str(node.left), label="True")
            if 0 <= node.right < len(nodes):
                dot.edge(str(i), str(node.right), label="False")
    dot.render(output_name, cleanup=True)
    dot.view()

if __name__ == "__main__":
    nodes = read_nodes_from_bin("iris_tree_nodes.bin")
    print(f"Loaded {len(nodes)} nodes from binary.")
    build_graphviz_tree(nodes)
