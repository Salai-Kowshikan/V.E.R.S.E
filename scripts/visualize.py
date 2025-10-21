from sklearn import tree
import graphviz
from sklearn.datasets import load_iris
from sklearn.tree import DecisionTreeClassifier
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType

# Load Iris dataset
iris = load_iris()
X, y = iris.data, iris.target

# Train Decision Tree
model = DecisionTreeClassifier(max_depth=3)
model.fit(X, y)

# If you still have the original Scikit-learn model:
dot_data = tree.export_graphviz(model, out_file=None, 
                                feature_names=iris.feature_names,
                                class_names=iris.target_names,
                                filled=True)
graph = graphviz.Source(dot_data)
graph.render("iris_tree")  # saves as iris_tree.pdf
graph.view()  # opens the diagram
