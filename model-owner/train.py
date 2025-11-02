import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.tree import DecisionTreeClassifier
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType
from sklearn.preprocessing import LabelEncoder
import onnx
import joblib

data = pd.read_csv("iris.csv")

X = data.iloc[:, :-1].values      
y = data.iloc[:, -1].values      

le = LabelEncoder()
y = le.fit_transform(y) 

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

model = DecisionTreeClassifier(max_depth=5)
model.fit(X_train, y_train)

# Convert to ONNX
initial_type = [('float_input', FloatTensorType([None, 4]))]
onnx_model = convert_sklearn(model, initial_types=initial_type)
onnx.save_model(onnx_model, "iris_tree.onnx")

# Save the trained model using joblib
# joblib.dump(model, "iris_tree_model.joblib")

print("Model training and conversion to ONNX completed.")