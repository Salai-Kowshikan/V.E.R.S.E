import onnx
from tabulate import tabulate

def print_onnx_tree(model_path, feature_names=None, class_names=None):
    # Load ONNX model
    model = onnx.load(model_path)
    onnx.checker.check_model(model)
    print("✅ Model Loaded Successfully!")

    # Print general model info
    print(f"IR version: {model.ir_version}")
    print(f"Producer name: {model.producer_name}")
    print(f"Opset version: {model.opset_import[0].version}")
    print(f"Number of nodes: {len(model.graph.node)}\n")

    # Print inputs
    print(" Inputs:")
    for inp in model.graph.input:
        dims = [d.dim_value if (d.dim_value > 0) else "None" for d in inp.type.tensor_type.shape.dim]
        print(f" - {inp.name} : {dims}")

    # Print node table
    rows = []
    for i, node in enumerate(model.graph.node):
        attr_dict = {a.name: onnx.helper.get_attribute_value(a) for a in node.attribute}
        rows.append([
            i + 1,
            node.op_type,
            node.name if node.name else "-",
            ", ".join(node.input),
            ", ".join(node.output),
            attr_dict
        ])
    print("\n🧩 Model Graph Nodes:")
    print(tabulate(rows, headers=["#", "OpType", "Name", "Inputs", "Outputs", "Attributes"], tablefmt="grid"))

    # Find TreeEnsembleClassifier node
    tree_node = None
    for node in model.graph.node:
        if node.op_type == "TreeEnsembleClassifier":
            tree_node = node
            break

    if not tree_node:
        print("\nNo TreeEnsembleClassifier found in the ONNX model.")
        return

    # Extract attributes safely
    attrs = {}
    for attr in tree_node.attribute:
        if attr.ints:
            attrs[attr.name] = list(attr.ints)
        elif attr.floats:
            attrs[attr.name] = list(attr.floats)
        elif attr.s:
            attrs[attr.name] = attr.s
        elif attr.strings:
            attrs[attr.name] = [s.decode() for s in attr.strings]
        else:
            attrs[attr.name] = None

    # Nodes and leaves
    nodes = {
        'ids': attrs['nodes_nodeids'],
        'feature': attrs['nodes_featureids'],
        'value': attrs['nodes_values'],
        'mode': attrs['nodes_modes'],
        'true': attrs['nodes_truenodeids'],
        'false': attrs['nodes_falsenodeids'],
    }
    leaves = {
        'ids': attrs['class_nodeids'],
        'tree': attrs['class_treeids'],
        'class_ids': attrs['class_ids'],
        'weights': attrs['class_weights']
    }

    # Map node_id -> children or leaf info
    node_map = {}
    for i, node_id in enumerate(nodes['ids']):
        node_map[node_id] = {
            'feature': nodes['feature'][i],
            'value': nodes['value'][i],
            'mode': nodes['mode'][i] if isinstance(nodes['mode'][i], str) else nodes['mode'][i].decode(),
            'true': nodes['true'][i],
            'false': nodes['false'][i]
        }

    leaf_map = {}
    for i in range(len(leaves['ids'])):
        node_id = leaves['ids'][i]
        class_id = leaves['class_ids'][i]
        weight = leaves['weights'][i]
        if node_id not in leaf_map:
            leaf_map[node_id] = {}
        leaf_map[node_id][class_id] = weight

    # Recursive print
    def recurse(node_id, indent=""):
        node = node_map.get(node_id)
        if node is None or node['mode'] == 'LEAF':
            probs = leaf_map.get(node_id, {})
            probs_str = []
            for k, v in probs.items():
                name = class_names[k] if class_names else str(k)
                probs_str.append(f"{name}: {v:.2f}")
            print(f"{indent}Leaf Node {node_id} → Probabilities: [{', '.join(probs_str)}]")
            return
        feature_name = feature_names[node['feature']] if feature_names else f"feature_{node['feature']}"
        print(f"{indent}Node {node_id}: {feature_name} <= {node['value']}")
        print(f"{indent}  ├─ True → ", end="")
        recurse(node['true'], indent + "  │   ")
        print(f"{indent}  └─ False → ", end="")
        recurse(node['false'], indent + "      ")

    print("\n🌳 TreeEnsembleClassifier Structure:")
    recurse(0)

feature_names = ["sepal_length", "sepal_width", "petal_length", "petal_width"]
class_names = ["setosa", "versicolor", "virginica"]

print_onnx_tree("iris_tree_copy.onnx", feature_names, class_names)
