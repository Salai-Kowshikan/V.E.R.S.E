from sklearn.tree import DecisionTreeClassifier
import numpy as np

# --- Step 1: Train a sample tree ---
def train_tree(X, y, max_depth=10):
    clf = DecisionTreeClassifier(max_depth=max_depth)
    clf.fit(X, y)
    return clf

# --- Step 2: Convert tree into arithmetic expression ---
def tree_to_arithmetic(tree, feature_names=None):
    """
    Recursively convert decision tree into arithmetic formula.
    """
    tree_ = tree.tree_

    def recurse(node):
        # Leaf node
        if tree_.feature[node] == -2:
            # Take the argmax of class counts for leaf
            leaf_value = np.argmax(tree_.value[node])
            return str(leaf_value)

        # Internal node
        feature_idx = tree_.feature[node]
        threshold = tree_.threshold[node]

        # Boolean branch b = 1 if feature <= threshold else 0
        b = f"(step({threshold} - x[{feature_idx}]))"

        left_expr = recurse(tree_.children_left[node])
        right_expr = recurse(tree_.children_right[node])

        # Arithmetic equivalent of if-else: b*left + (1-b)*right
        expr = f"({b}*({left_expr}) + (1-{b})*({right_expr}))"
        return expr

    return recurse(0)

# --- Step 3: Example usage ---
if __name__ == "__main__":
    from sklearn.datasets import load_iris

    iris = load_iris()
    X, y = iris.data, iris.target

    clf = train_tree(X, y, max_depth=3)
    arithmetic_expr = tree_to_arithmetic(clf)

    print("Arithmetic expression representing the decision tree:\n")
    print(arithmetic_expr)
