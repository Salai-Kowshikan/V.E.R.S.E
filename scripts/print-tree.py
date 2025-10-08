import onnx

# Load the ONNX model
model = onnx.load("iris_tree.onnx")

# Print a readable summary of the graph
print(onnx.helper.printable_graph(model.graph))


for node in model.graph.node:
    if node.op_type == "TreeEnsembleClassifier":
        print("Attributes of Decision Tree:")
        for attr in node.attribute:
            print(attr.name, ":", attr)
