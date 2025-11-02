import onnxruntime as ort

session = ort.InferenceSession("iris_tree_model.onnx")
print("Inputs:", [i.name for i in session.get_inputs()])
print("Outputs:", [o.name for o in session.get_outputs()])
