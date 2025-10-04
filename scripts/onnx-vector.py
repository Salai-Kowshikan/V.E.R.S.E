import pickle
from dataclasses import dataclass
from typing import List
from sklearn.datasets import load_iris
from sklearn.tree import DecisionTreeClassifier

@dataclass
class Node:
    feature_index: int
    threshold: float
    left: int
    right: int
    class_label: int  # -1 if not a leaf

def sklearn_tree_to_nodes(tree: DecisionTreeClassifier) -> List[Node]:
    nodes: List[Node] = []
    tree_ = tree.tree_

    for i in range(tree_.node_count):
        feature = tree_.feature[i]
        threshold = tree_.threshold[i]
        left = tree_.children_left[i]
        right = tree_.children_right[i]
        if left == -1 and right == -1:
            # Leaf node
            class_label = int(tree_.value[i].argmax())
        else:
            class_label = -1
        nodes.append(Node(feature, threshold, left, right, class_label))
    return nodes

def main():
    # Example: train a DecisionTree on Iris
    iris = load_iris()
    X, y = iris.data, iris.target
    clf = DecisionTreeClassifier()
    clf.fit(X, y)

    nodes = sklearn_tree_to_nodes(clf)

    # Save to binary file
    with open("iris_tree_nodes.bin", "wb") as f:
        f.write(pickle.dumps(nodes))

    print(f"Serialized {len(nodes)} nodes to iris_tree_nodes.bin")

if __name__ == "__main__":
    main()
