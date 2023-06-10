import tensorflow as tf
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score


mnist = tf.keras.datasets.mnist
# Załaduj zbiór danych MNIST
(x_train, y_train), (x_test, y_test) = mnist.load_data()

# Przygotuj dane treningowe
x_train = x_train.reshape(-1, 28 * 28) / 255.0
x_test = x_test.reshape(-1, 28 * 28) / 255.0

# Stwórz i wytrenuj las decyzyjny
clf = RandomForestClassifier(n_estimators=100, random_state=42)
clf.fit(x_train, y_train)

# Dokonaj predykcji na zbiorze testowym
y_pred = clf.predict(x_test)

# Oblicz współczynnik prawidłowej rozpoznawalności
accuracy = accuracy_score(y_test, y_pred)
print("Współczynnik prawidłowej rozpoznawalności lasu decyzyjnego:", accuracy)
