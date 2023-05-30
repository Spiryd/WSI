### ZAD 3

##IMPL OF NN WITH K HIDDEN NODES (ONE LAYER), CHOICE OF NO, L1, L2, AND CHOICE RELU VS SIGMOID

from random import seed
from random import randrange
from random import random
from csv import reader
from math import exp
 
# Split a dataset into k and mix them up
def cross_validation_split(dataset, n_folds):

 dataset_split = list()
 dataset_copy = list(dataset)

 fold_size = int(len(dataset) / n_folds)

 for i in range(n_folds):
  fold = list()

  while len(fold) < fold_size:
    index = randrange(len(dataset_copy))
    fold.append(dataset_copy.pop(index))

  dataset_split.append(fold)

 return dataset_split
 
# Calculate accuracy percentage
def accuracy_metric(actual, predicted):
 correct = 0

 for i in range(len(actual)):
  if actual[i] == predicted[i]:
    correct += 1

 return correct / float(len(actual)) * 100.0
 
# Evaluate an algorithm using a cross validation split
def evaluate_algorithm(dataset, algorithm, n_folds, normalization_method, *args):

 folds = cross_validation_split(dataset, n_folds)
 scores = list()

 for fold in folds:
  train_set = list(folds)
  train_set.remove(fold)
  train_set = sum(train_set, [])
  test_set = list()

  for row in fold:
    row_copy = list(row)
    test_set.append(row_copy)
    row_copy[-1] = None

  predicted = algorithm(train_set, test_set, normalization_method, *args)
  actual = [row[-1] for row in fold]
  accuracy = accuracy_metric(actual, predicted)
  scores.append(accuracy)

 return scores
 
# Calc all neuron activation values
def activate(weights, inputs):
 activation = weights[-1]


 for i in range(len(weights)-1):
  activation += weights[i] * inputs[i]
 return activation
 
# SIGMOID ACTIVATION FUNCTION
def sigmoid(activation):
 try:
    result = 1.0 / (1.0 + exp(-activation))
 except OverflowError:
    result = float('inf')
 return result

# RELU ACTIVATION FUNCTION
def ReLU(activation):
    try:
        result = 1.0 / (1.0 + exp(-activation))
    except OverflowError:
        result = float('inf')
    return result
 
def forward_propagate(network, row):
 inputs = row
 for layer in network:
  new_inputs = []
  for neuron in layer:
    activation = activate(neuron['weights'], inputs)
    
    #USE SIGMOID
    neuron['output'] = sigmoid(activation)
    #USE RELU 
    #neuron['output'] = ReLU(activation)

    new_inputs.append(neuron['output'])
  inputs = new_inputs
 return inputs
 
def transfer_derivative(output):
 return output * (1.0 - output)
 
# Backpropagate error 
def backward_propagate_error(network, expected):
 for i in reversed(range(len(network))):
  layer = network[i]
  errors = list()
  if i != len(network)-1:
    for j in range(len(layer)):
      error = 0.0
      for neuron in network[i + 1]:
        error += (neuron['weights'][j] * neuron['delta'])
      errors.append(error)
  else:
    for j in range(len(layer)):
      neuron = layer[j]

      error = neuron['output'] - expected[j]

      norm_lambda1 = 0.001
      norm_lambda2 = 0.0001

      l1_norm = sum(sum(sum(abs(weight) for weight in neuron['weights']) for neuron in layer) for layer in network)
      l2_norm = sum(sum(sum(weight**2 for weight in neuron['weights']) for neuron in layer) for layer in network)

      if normalization_method == 0:
        continue
      elif normalization_method == 1:
        error += norm_lambda1 * l1_norm
      elif normalization_method == 2:
        error += norm_lambda2 * l2_norm

      errors.append(error)

  for j in range(len(layer)):
    neuron = layer[j]
    neuron['delta'] = errors[j] * transfer_derivative(neuron['output'])
 
# Update network weights with error
def update_weights(network, row, l_rate):
 for i in range(len(network)):
  inputs = row[:-1]
  if i != 0:
    inputs = [neuron['output'] for neuron in network[i - 1]]
  for neuron in network[i]:
    for j in range(len(inputs)):
      neuron['weights'][j] -= l_rate * neuron['delta'] * inputs[j]
    neuron['weights'][-1] -= l_rate * neuron['delta']
 
# Train a network for a fixed number of epochs
def train_network(network, train, l_rate, n_epoch, n_outputs, normalization_method):
 for epoch in range(n_epoch):
  sum_error = 0
  for row in train:
    outputs = forward_propagate(network, row)
    expected = [0 for i in range(n_outputs)]
    expected[row[-1]] = 1
    
    #mean squared error
    sum_error += sum([(expected[i]-outputs[i])**2 for i in range(len(expected))])

    backward_propagate_error(network, expected)
    update_weights(network, row, l_rate)
  print('>epoch=%d, lrate=%.3f, error=%.3f' % (epoch, l_rate, sum_error))
 
# Initialize a network
def initialize_network(n_inputs, n_hidden, n_outputs):
 network = list()
 hidden_layer = [{'weights':[random.random() for i in range(n_inputs + 1)], 'delta': 1} for i in range(n_hidden)]
 network.append(hidden_layer)
 output_layer = [{'weights':[random.random() for i in range(n_hidden + 1)], 'delta': 1} for i in range(n_outputs)]
 network.append(output_layer)
 return network
 
# Make a prediction with a network
def predict(network, row):
 outputs = forward_propagate(network, row)
 return outputs.index(max(outputs))
 
# Backpropagation Algorithm With Stochastic Gradient Descent
def back_propagation(train, test,normalization_method,  l_rate, n_epoch, n_hidden):
 n_inputs = len(train[0]) - 1
 n_outputs = len(set([row[-1] for row in train]))   #[0, 1]
 print(n_outputs)
 network = initialize_network(n_inputs, n_hidden, n_outputs)
 train_network(network, train, l_rate, n_epoch, n_outputs, normalization_method)
 predictions = list()
 for row in test:
  prediction = predict(network, row)
  predictions.append(prediction)
 return(predictions)
 
#EXPERIMENT PART

import random

def generate_dataset(data_points):
  dataset = []

  for index in range(0, data_points):
    v = []
    first = random.uniform(-1, 1)
    second = random.uniform(-1, 1)

    if first == 0 or second == 0:
      first = random.uniform(-1, 1)
      second = random.uniform(-1, 1)

    expected = 0 if (first * second) > 0 else 1

    v.append(first)
    v.append(second)
    v.append(expected)
    dataset.append(v)

  return dataset

# SETTINGS
n_folds = 5
l_rate = .7
n_epoch = 10
n_hidden = 4

normalization_method = 1
normalization_lambda = 0.001

# Prepare the data

dataset = generate_dataset(1000)

# evaluate algorithm
scores = evaluate_algorithm(dataset, back_propagation, n_folds, normalization_method, l_rate, n_epoch, n_hidden)
print('Scores: %s' % scores)
print('Mean Accuracy: %.3f%%' % (sum(scores)/float(len(scores))))

