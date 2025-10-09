import onnx
from onnx.tools.net_drawer import GetPydotGraph, GetOpNodeProducer

# Load ONNX model
model = onnx.load("iris_tree.onnx")

onnx.checker.check_model(model)
print("✅ Model is valid and loadable.")

# Create graph visualization
pydot_graph = GetPydotGraph(model.graph, name=model.graph.name, rankdir="TB", node_producer=GetOpNodeProducer("docstring"))
pydot_graph.write_pdf("onnx_graph.pdf")
