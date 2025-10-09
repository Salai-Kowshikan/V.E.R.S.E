import onnx
import numpy as np

def print_tree(onnx_model_path, feature_names=None, class_names=None):
    # Load ONNX model
    model = onnx.load(onnx_model_path)
    
    # Find TreeEnsembleClassifier node
    tree_node = None
    for node in model.graph.node:
        if node.op_type == "TreeEnsembleClassifier":
            tree_node = node
            break
    if tree_node is None:
        print("No TreeEnsembleClassifier found in the model.")
        return

    # Extract attributes
    attrs = {}
    for attr in tree_node.attribute:
        if attr.ints:
            attrs[attr.name] = list(attr.ints)
        elif attr.floats:
            attrs[attr.name] = list(attr.floats)
        elif attr.strings:
            attrs[attr.name] = [s.decode() for s in attr.strings]
        elif attr.HasField('s'):
            attrs[attr.name] = attr.s.decode()
        else:
            attrs[attr.name] = None

    # Default feature and class names
    if feature_names is None:
        feature_names = [f"feature_{i}" for i in range(max(attrs['nodes_featureids']) + 1)]
    if class_names is None:
        class_names = [str(c) for c in attrs['classlabels_int64s']]

    # Build node dictionary
    nodes = {}
    for i, node_id in enumerate(attrs['nodes_nodeids']):
        nodes[node_id] = {
            'feature': attrs['nodes_featureids'][i],
            'threshold': attrs['nodes_values'][i],
            'true_id': attrs['nodes_truenodeids'][i],
            'false_id': attrs['nodes_falsenodeids'][i],
            'mode': attrs['nodes_modes'][i],
        }

    # Map leaf node class probabilities
    leaf_probs = {}
    for c_id, n_id, w in zip(attrs['class_ids'], attrs['class_nodeids'], attrs['class_weights']):
        if n_id not in leaf_probs:
            leaf_probs[n_id] = np.zeros(len(class_names))
        leaf_probs[n_id][c_id] = w

    # Recursive function to print tree
    def recurse(node_id, indent=""):
        node = nodes[node_id]
        if node['mode'] == "LEAF":
            probs = leaf_probs.get(node_id, None)
            probs_str = ", ".join([f"{class_names[i]}: {p:.2f}" for i, p in enumerate(probs)]) if probs is not None else "N/A"
            print(f"{indent}Leaf Node {node_id} → Probabilities: [{probs_str}]")
        else:
            feature = feature_names[node['feature']] if node['feature'] < len(feature_names) else f"feature_{node['feature']}"
            threshold = node['threshold']
            print(f"{indent}Node {node_id}: {feature} <= {threshold}")
            recurse(node['true_id'], indent + "  ├─ True → ")
            recurse(node['false_id'], indent + "  └─ False → ")

    # Start from root node (0)
    recurse(0)

# Example usage
print("\n\nModel 1:\n\n")
print_tree(
    "iris_tree.onnx",
    feature_names=["sepal_length","sepal_width","petal_length","petal_width"],
    class_names=["setosa","versicolor","virginica"]
)
print("\n\nModel 2:\n\n")
print_tree(
    "iris_tree_copy.onnx",
    feature_names=["sepal_length","sepal_width","petal_length","petal_width"],
    class_names=["setosa","versicolor","virginica"]
)
