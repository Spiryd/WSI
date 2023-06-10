import tensorflow as tf
from sklearn.metrics import accuracy_score, confusion_matrix

(x_train, y_train), (x_test, y_test) = tf.keras.datasets.mnist.load_data()

x_train = x_train.reshape(-1, 28 * 28) / 255.0
x_test = x_test.reshape(-1, 28 * 28) / 255.0

#feature scaling
from sklearn.preprocessing import StandardScaler
scaler = StandardScaler()
x_train = scaler.fit_transform(x_train)
x_test = scaler.fit_transform(x_test)

#PCA
from sklearn.decomposition import PCA
pca = PCA(n_components=2, random_state=0)
train_pca = pca.fit_transform(x_train, y_train)
test_pca = pca.fit_transform(x_test, y_test)

#PCA -> DBSCAN
from sklearn.cluster import DBSCAN
eps = 0.5
min_samples = 20
print(f"eps = {eps}, min_samples = {min_samples}")
db = DBSCAN(eps=eps, min_samples=min_samples)
db = db.fit(train_pca)
labels_db = db.labels_
clusters_db = db.fit_predict(test_pca)
n_clusters_ = len(set(clusters_db)) - (1 if -1 in clusters_db else 0)
print ("number of clusters in pca-DBSCAN: ", n_clusters_)
noise_percentage = (clusters_db == -1).sum() / len(clusters_db) * 100
print("Procent szumu:", noise_percentage)


### Evaluation Metrics
# ARI(adjusted rand index)
from sklearn.metrics.cluster import adjusted_rand_score
# y_test / clusters_km compare
print ("ARI of training set: {:.2f}".format(adjusted_rand_score(y_test, clusters_db)))