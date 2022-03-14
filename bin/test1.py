# import math as np
# import time

# from array import array
# def abs(a):
#     number = 0
#     if a < 0:
#         number = -a
#     else:
#         number = a
#     return number

# def listabs(list):
#     list1 = []
#     for i in range(len(list)):
#         if list[i] < 0:

#             list1.append(-list[i])
#         else:
#             list1.append(list[i])

#     return list1

# def square(b):
#     return b*b

# def argmax(list):
#     now = list[0]
#     index = 0
#     for i in range(len(list)):
#         if list[i]>now:
#             now = list[i]
#             index = i
#     return index

# class KNN:
#     def __init__(self, K, metric_type):
#         """
#         Args:
#             K (int): K-value
#             metric_type (str): L1, L2, or L-inf
#         """
#         self.K = K
#         self.metric_type = metric_type

#     def distance_func(self, vec1, vec2):
#         """
#         Computes the distance between two d-dimension vectors.

#         Args:
#             vec1 ((d,) np.ndarray): d-dim vector
#             vec2 ((d,)) np.ndarray): d-dim vector
#         """



#         c = []

#         for i in range(len(vec1)):
#             d = []
#             for j in range(len(vec2)):
#                 d1 = vec1[i][j] - vec2[j]
#                 d.append(d1)
#             c.append(d)
#         diff = c


#         # diff = vec1 - vec2

#         if self.metric_type == "L1":
#             # write your code her

#             distance = []
#             for i in range(len(vec1)):
#                 sum1 = 0
#                 for j in range(len(vec1[i])):
#                     sum1+= abs(diff[i][j])
#                 distance.append(sum1)


#         elif self.metric_type == "L2":
#             # write your code here
#             distance = []
#             for i in range(len(vec1)):
#                 sum1 = 0
#                 for j in range(len(vec1[i])):
#                     sum1 += square(abs(diff[i][j]))

#                 sum1 = np.sqrt(sum1)
#                 distance.append(sum1)

#         elif self.metric_type == "L-inf":
#             # write your code here
#             distance = []
#             for i in range(len(vec1)):


#                 sum1 = max(listabs(diff[i]))
#                 distance.append(sum1)

#         return distance

#     def fit(self, X_train, y_train):
#         """
#         Args:
#             X_train ((n,d) np.ndarray): training data with n samples and d features
#             y_train ((n,) np.ndarray): training labels
#         """
#         self.X_train = X_train
#         self.y_train = y_train

#     def compute_distances_neighbors(self, sample):
#         """
#         Computes the distance between every data point in the train set and the 
#         given sample and then finds the k-nearest neighbors.

#         Returns a numpy array of the labels of the k-nearest neighbors.

#         Args:
#             sample ((d,) np.ndarray): the given sample to be computed

#         Returns:
#             neighbors (list): K-nearest neighbors' labels
#         """

#         # write your code here
#         distance = self.distance_func(self.X_train,sample)

#         neighbors = []
#         temp = []
#         Inf = 1000000000
#         for i in range(self.K):
#             temp.append(distance.index(min(distance)))
#             neighbors.append(self.y_train[distance.index(min(distance))])

#             distance[distance.index(min(distance))] = Inf

#         return neighbors


#     @staticmethod
#     def majority(neighbors):
#         """
#         Performs majority voting and returns the predicted value for the test sample.
#         Since we're performing binary classification, the possible values are [0,1].

#         Args:
#             neighbors (list): K-nearest neighbors' labels

#         Returns:
#             predicted_value (int): the predicted label for the given sample
#         """

#         # write your code here

#         list = [0 for index in range(max(neighbors)+1)]
#         for i in range(len(neighbors)):

#             list[neighbors[i]] = list[neighbors[i]]+1



#         a = argmax(list)
#         return a



#     def predict(self, X_test):
#         """
#         Computes the predicted values for the entire test set.

#         Args:
#             X_train ((n,d) np.ndarray): training data with n samples and d features
#             y_train ((n,) np.ndarray): training labels
#             X_test ((n,d) np.ndarray): test data

#         Returns:
#             pred_test ((n,) np.ndarray): output for every entry in the test set
#         """
#         list = []
#         for sample in X_test:
#             list.append(self.majority(self.compute_distances_neighbors(sample)))
#         pred_test = list


#         return pred_test


# def accuracy_score(pred, y):
#     """
#     Computes the accuracy of the predicted data.

#     Args:
#         pred ((n,) np.ndarray): predicted values for n samples
#         y ((n,) np.ndarray): labels for n samples

#     Returns:
#         acc (float): accuracy
#     """

#     sum = len(pred)
#     true = 0
#     for i in range(len(pred)):
#         if pred[i] == y[i]:
#             true = true+1

#     acc = true/sum




#     return acc


# def main():
#     time1 = time.time()
#     open_diff = open('sj.txt', 'r')
    
#     diff_line = open_diff.readlines()
#     print("read file time:",(time.time()-time1)," s")
#     n, m, d = diff_line[0].split()

    
#     x_train = []
#     x_test = []
#     y_train = []
#     y_test = []
#     for i in range(1,int(n)+1):
#         split_line = diff_line[i].split()
#         X = split_line[:int(d)]
#         y = split_line[-1]
#         x_train.append([float(x) for x in X])
#         y_train.append(int(y))

#     for i in range(int(n)+1,int(m)+int(n)+1):
#         split_line = diff_line[i].split()
#         X = split_line[:int(d)]
#         y = split_line[-1]
#         x_test.append([float(x) for x in X])
#         y_test.append(int(y))

#     # x_train = np.array(x_train)
#     # x_test = np.array(x_test)
#     # y_train = np.array(y_train)
#     # y_test = np.array(y_test)


    
#     metric_types = ["L1", "L2", "L-inf"]
#     params = [(x, y) for x in range(1, 6, 1) for y in metric_types]

#     time1 = time.time()
#     list = []
#     list1 = []
#     list2 = []
#     for i in range(len(params)):
#         list2.append(params[i][0])
#         list1.append(params[i][1])
#         model = KNN(params[i][0],params[i][1])
#         model.fit(x_train,y_train)

#         y_pre = model.predict(x_test)
#         list.append(accuracy_score(y_pre,y_test))

#     print("train time:",(time.time()-time1)," s")
#     for i in range(len(list1)):
#         if list[i] == max(list):
#             print(list2[i],list1[i])

#     for i in range(len(list1)):
#         print(list2[i],list1[i],list[i])

    





#     # write your code here

# if __name__ == '__main__':
#     time1 = time.time()
#     main()
#     time2 = time.time()
#     print("time:",(time2-time1)," s")
    