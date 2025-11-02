import json

# Load the tree from a JSON file
with open("tree.json", "r") as f:
    tree_json = json.load(f)

# Convert list to dict for fast lookup
tree = {node['id']: node for node in tree_json}

def predict(features, node_id=0):
    node = tree[node_id]
    # Leaf node
    if node['feature'] is None:
        return node['value'][0].index(max(node['value'][0]))
    # Traverse left or right
    if features[node['feature']] <= node['threshold']:
        return predict(features, node['left'])
    else:
        return predict(features, node['right'])


# Verification samples
samples = [
    {"features":[5.1, 3.5, 1.4, 0.2], "expected":0},
    {"features":[4.9, 3.0, 1.4, 0.2], "expected":0},
    {"features":[6.0, 2.2, 4.0, 1.0], "expected":1},
    {"features":[5.9, 3.0, 5.1, 1.8], "expected":2},
    {"features":[6.5, 3.0, 5.2, 2.0], "expected":2},
]

# Run predictions
for s in samples:
    pred = predict(s["features"])
    print(f"Features: {s['features']}, Predicted: {pred}, Expected: {s['expected']}")
