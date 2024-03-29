import os
import cv2
import tensorflow as tf
import numpy as np

def gen_my_test_data():
    directory = 'test_images'
    y = [i for i in range(10) for _ in range(10)]
    x = []
    for filename in os.scandir(directory):
        if filename.is_file():
            img = cv2.imread(filename.path, 0)
            pic = []
            for i in range (img.shape[0]): #traverses through height of the image
                row = []
                for j in range (img.shape[1]): #traverses through width of the image
                    row.append(1 - img[i][j]/255)
                pic.append(row)
            x.append(pic)
    return np.array(x), np.asarray(y)

my_test_x, my_test_y = gen_my_test_data()

# Step 1: Load MNIST dataset
mnist = tf.keras.datasets.mnist
(x_train, y_train), (x_test, y_test) = mnist.load_data()

# Step 2: Preprocess the data
x_train, x_test = x_train / 255.0, x_test / 255.0

# Step 3: Define the neural network architecture
model = tf.keras.models.Sequential([
    tf.keras.layers.Flatten(input_shape=(28, 28)),
    tf.keras.layers.Dense(128, activation='relu'),
    tf.keras.layers.Dense(10, activation='softmax')
])

# Step 4: Compile the model
model.compile(optimizer='adam',
              loss='sparse_categorical_crossentropy',
              metrics=['accuracy'])

# Step 5: Train the model
model.fit(x_train, y_train, epochs=5)

# Step 6: Evaluate the model
test_loss, test_acc = model.evaluate(x_test, y_test)
print(f"Standard Test accuracy: {test_acc}")

test_loss, test_acc = model.evaluate(my_test_x, my_test_y)
print(f"My Test accuracy: {test_acc}")
