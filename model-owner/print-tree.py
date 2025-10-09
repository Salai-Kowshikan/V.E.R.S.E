import onnx
from tabulate import tabulate

# Load ONNX model
# model_path = "iris_tree.onnx"
model_path = "iris_tree_copy.onnx"
model = onnx.load(model_path)

# Validate model
onnx.checker.check_model(model)

# Print general model info
print("✅ Model Loaded Successfully!")
print(f"IR version: {model.ir_version}")
print(f"Producer name: {model.producer_name}")
print(f"Opset version: {model.opset_import[0].version}")
print(f"Number of nodes: {len(model.graph.node)}\n")

# Print inputs and outputs
print(" Inputs:")
for inp in model.graph.input:
    print(f" - {inp.name} : {[d.dim_value for d in inp.type.tensor_type.shape.dim]}")

# print("\n Outputs:")
# for out in model.graph.output:
#     print(f" - {out.name} : {[d.dim_value for d in out.type.tensor_type.shape.dim]}")

# Print all node details in a readable table
rows = []
for i, node in enumerate(model.graph.node):
    rows.append([
        i + 1,
        node.op_type,
        node.name if node.name else "-",
        ", ".join(node.input),
        ", ".join(node.output),
        str({a.name: onnx.helper.get_attribute_value(a) for a in node.attribute})
    ])

print("\n🧩 Model Graph Nodes:")
print(tabulate(rows, headers=["#", "OpType", "Name", "Inputs", "Outputs", "Attributes"], tablefmt="grid"))


