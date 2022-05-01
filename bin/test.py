import pickle
import numpy as np
import math
import time

def Parameter_initialization(num_input,num_layer1,num_layer2,num_output):

    Parameter={}
    #Your code starts here
    # Please Initialize all parameters used in ANN-Hidden Layers with Xavier
    w1 = xavier(np.zeros((num_input,num_layer1)))
    b1 = xavier(np.zeros((1,num_layer1)))
    w2 = xavier(np.zeros((num_layer1,num_layer2)))
    b2 = xavier(np.zeros((1,num_layer2)))
    w3 = xavier(np.zeros((num_layer2,num_output)))
    b3 = xavier(np.zeros((1,num_output)))
    #Your code ends here
    Parameter['w1']=w1
    Parameter['b1']=b1
    Parameter['w2']=w2
    Parameter['b2']=b2
    Parameter['w3']=w3
    Parameter['b3']=b3
    return Parameter

def Hidden_Layer(x,w,b,batch_size):
    a = np.dot(x,w)
    shape = a.shape
    for batch in range(batch_size):
        
        for i in range(shape[1]):
            a.set(batch,i,a[batch,i]+b[0,i])
    return relu(a)
def Output_Layer(x,w,b,batch_size):
    # Your code starts here
    a = np.dot(x,w)
    shape = a.shape
    for batch in range(batch_size):
        for i in range(shape[1]):
            a.set(batch,i,a[batch,i]+b[0,i])
    a = softmax(a)
    # Your code ends here
    return a

def Loss(lable, logits):
    # lable : Actual label  BATCHSIZE *class
    # logits : The predicted results of your model
    # Your code starts here
    loss_tmp = 0
    logits = np.log(logits)
    loss_tmp = -np.sum(lable*logits)/lable.shape[0]
    # Your code ends here
    return loss_tmp
def Back_propagation(logits,label,w1,b1,w2,b2,w3,b3,a2,a1,image_blob):
    # lable : Actual label  BATCHSIZE *class
    # logits : The predicted results of your model
    # Your code starts here
    tmp = logits-label
    batch_size = logits.shape[0]
    d_w3 = np.dot(np.transpose(a2),tmp)
    d_b3 = np.transpose(-np.dot(np.transpose(tmp),np.ones((batch_size,1))))


    tmp = np.dot(tmp,np.transpose(w3))
    tmp = tmp*d_relu(a2)

    d_w2 = np.dot(np.transpose(a1),tmp)
    d_b2 = np.transpose(-np.dot(np.transpose(tmp),np.ones((batch_size,1))))

    tmp = np.dot(tmp,np.transpose(w2))
    tmp = tmp*d_relu(a1)

    d_w1 = np.dot(np.transpose(image_blob),tmp)
    d_b1 = np.transpose(-np.dot(np.transpose(tmp),np.ones((batch_size,1))))

    # Your code ends here
    return  d_w1,d_b1,d_w2,d_b2,d_w3,d_b3

def xavier(ndarray):
    n1 = ndarray.shape[0]
    n2 = ndarray.shape[1]
    x = math.sqrt(6*1.0/(n1 + n2))
    ndarray = np.random_uniform(-x, x, (n1, n2))
    return ndarray

def relu(ndarray):
    return np.maximum(ndarray, 0)

def d_relu(ndarray):
    temp = np.minimum_or_value(np.maximum(ndarray,0),0,1.0)
    return temp

def softmax(ndarray):
    r = ndarray.shape[0]
    c = ndarray.shape[1]
    max_arr = np.tile(ndarray.max(1), (1,c))
    e_x = np.exp(ndarray - max_arr)
    return e_x / np.tile(e_x.sum(1), (1,c))

def getLabel(ndarray,BATCHSIZE,num_output):
    label = np.zeros((BATCHSIZE, num_output))
    for i in range(0, BATCHSIZE):
        idx = ndarray[i,0]
        label.set(i,int(idx),1.0)
    return label

if __name__ == '__main__':
    train_images = pickle.load(open('train_images.pkl','rb'))
    test_images = pickle.load(open('test_images.pkl','rb'))
    train_labels = pickle.load(open('train_labels.pkl','rb'))
    test_labels = pickle.load(open('test_labels.pkl','rb'))
    test_labels = np.array(test_labels)
    test_images = np.array(test_images)
    train_images = np.array(train_images)
    train_labels = np.array(train_labels)
    # test_labels = np.ones((100,1))
    # test_images = np.ones((100,784))
    # train_images = np.ones((1000,784))
    # train_labels = np.ones((1000,1))
    start_time = time.time()
    EPOCH = 20
    ITERS = 10
    BATCHSIZE = 100
    LR_BASE = 0.1
    k = 0.0005  # lambda
    num_input = 784
    num_layer1 = 300
    num_layer2 = 100
    num_output = 10
    ### 1. Data preprocessing: normalize all pixels to [0,1) by dividing 256
    train_images = train_images/256.0
    test_images = test_images/256.0
    # print(type(train_images[0][0]))

    ### 2. Weight initialization: Xavier
    Parameter = Parameter_initialization(num_input,num_layer1,num_layer2,num_output)
    w1,b1,w2,b2,w3,b3=Parameter['w1'],Parameter['b1'],Parameter['w2'],Parameter['b2'],Parameter['w3'],Parameter['b3']

    ### 3. training of neural network
    loss = np.zeros((EPOCH,1))   #save the loss of each epoch
    accuracy = np.zeros((EPOCH,1))  #save the accuracy of each epoch
    for epoch in range(0, EPOCH):
        if epoch <= EPOCH/2:
            lr = LR_BASE
        else:
            lr = LR_BASE / 10.0
        for iters in range(0, ITERS):
            # print("epoch:"+str(epoch)+"iters:"+str(iters))
            image_blob = train_images[iters*BATCHSIZE:(iters+1)*BATCHSIZE, :] # 100*784
            label_blob = train_labels[iters*BATCHSIZE:(iters+1)*BATCHSIZE, :] # 100*1
            label = getLabel(label_blob,BATCHSIZE,num_output)

            # Forward propagation  Hidden Layer
            a1 = Hidden_Layer(image_blob,w1,b1,BATCHSIZE)
            a2 = Hidden_Layer(a1,w2,b2,BATCHSIZE)
            
            # Forward propagation  output Layer
            a3 = Output_Layer(a2, w3, b3, BATCHSIZE)
            
            if np.count_nonzero(a3) != 1000:
                print(a3)
            #comupte loss
            
            loss_tmp = Loss(label,a3)
            if iters % 10 == 9:
                loss.set(epoch,0 , loss_tmp)
                print('Epoch '+str(epoch+1)+': ')
                print(loss_tmp)
            # Back propagation
            d_w1,d_b1,d_w2,d_b2,d_w3,d_b3= Back_propagation(a3,label,w1,b1,w2,b2,w3,b3,a2,a1,image_blob)
            
            # Gradient update
            w1 = w1 - (d_w1/BATCHSIZE)*lr - w1*(lr*k)
            b1 = b1 - (d_b1/BATCHSIZE)*lr
            w2 = w2 - (d_w2/BATCHSIZE)*lr - w2*(lr*k)
            b2 = b2 - (d_b2/BATCHSIZE)*lr
            w3 = w3 - (d_w3/BATCHSIZE)*lr - w3*(lr*k)
            b3 = b3 - (d_b3/BATCHSIZE)*lr
            

            
            # Testing for accuracy
            if iters % 10 == 9:
                z1 = np.dot(test_images, w1) + np.tile(b1, (100,1))
                a1 = relu(z1)
                z2 = np.dot(a1, w2) + np.tile(b2, (100,1))
                a2 = relu(z2)
                z3 = np.dot(a2, w3) + np.tile(b3, (100,1))
                a3 = softmax(z3) # 1000*10
                predict = np.argmax(a3, 1)
                print('Accuracy: ')
                accuracy.set(epoch,0 ,1 - np.count_nonzero(predict - test_labels)*1.0/100)
                print(accuracy[epoch,0])

    ### 4. Plot
    # print(loss)
    # print(accuracy)
    print("time: ",time.time()-start_time,"s")
    # plt.figure(figsize=(12,5))
    # ax1 = plt.subplot(121)
    # ax1.plot(np.arange(EPOCH)+1, loss[0:], 'r', label='Loss', linewidth=2)
    # plt.xlabel('Epoch', fontsize=16)
    # plt.ylabel('Loss on trainSet', fontsize=16)
    # plt.grid()
    # ax2 = plt.subplot(122)
    # ax2.plot(np.arange(EPOCH)+1, accuracy[0:], 'b', label='Accuracy', linewidth=2)
    # plt.xlabel('Epoch', fontsize=16)
    # plt.ylabel('Accuracy on trainSet', fontsize=16)
    # plt.grid()
    # plt.tight_layout()
    # plt.savefig('figure.pdf', dbi=300)
    # plt.show()
